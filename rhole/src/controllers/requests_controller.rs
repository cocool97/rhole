use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use hickory_client::{
    op::{MessageType, Query, ResponseCode},
    rr::{DNSClass, RData, RecordType},
};
use hickory_resolver::{
    config::{NameServerConfig, ResolverConfig, ResolverOpts},
    lookup::Lookup,
    name_server::{GenericConnector, TokioRuntimeProvider},
    AsyncResolver, Hosts, Name,
};
use hickory_server::{
    authority::MessageResponseBuilder,
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};

use crate::{
    api_models::LiveRequest,
    models::{dns_default_response, ProxyServer},
};

use super::{BlacklistController, DatabaseController, WatcherController};

pub fn get_static_hosts(local_hosts: &HashMap<String, Ipv4Addr>) -> Option<Hosts> {
    let mut hosts = Hosts::new();
    let mut nb: u8 = 0;
    for (host, addr) in local_hosts {
        let name = match Name::from_str(host) {
            Ok(name) => name,
            Err(e) => {
                log::error!("Error: {e}");
                continue;
            }
        };

        let mut q = Query::new();
        q.set_name(name.clone());
        q.set_query_class(DNSClass::IN);
        q.set_query_type(RecordType::A);

        hosts.insert(
            name,
            RecordType::A,
            Lookup::from_rdata(q, RData::A(hickory_client::rr::rdata::A(*addr))),
        );

        log::debug!("Adds static-host {host}: {addr}");
        nb += 1;
    }

    log::info!("Found {} static hosts...", nb);

    Some(hosts)
}

pub struct RequestsController {
    resolver: AsyncResolver<GenericConnector<TokioRuntimeProvider>>,
    blacklist_controller: BlacklistController,
    live_requests_controller: WatcherController<Option<LiveRequest>, u32>,
    database_controller: DatabaseController,
}

impl RequestsController {
    pub async fn new(
        proxy: ProxyServer,
        cache_size: Option<usize>,
        blacklist_controller: BlacklistController,
        local_hosts: &HashMap<String, Ipv4Addr>,
        live_requests_controller: WatcherController<Option<LiveRequest>, u32>,
        database_controller: DatabaseController,
    ) -> Result<Self> {
        let name_server_config = NameServerConfig {
            socket_addr: std::net::SocketAddr::new(
                std::net::IpAddr::V4(proxy.ip.parse()?),
                proxy.port,
            ),
            protocol: hickory_resolver::config::Protocol::Tls,
            tls_dns_name: Some(proxy.tls_dns_name.clone()),
            trust_negative_responses: true,
            tls_config: None,
            bind_addr: None,
        };

        let mut resolver_config = ResolverConfig::new();
        resolver_config.add_name_server(name_server_config);

        let mut resolver_opts = ResolverOpts::default();
        if let Some(cache_size) = cache_size {
            resolver_opts.cache_size = cache_size;
        }

        let mut resolver = AsyncResolver::tokio(resolver_config, resolver_opts);

        // Sets static hosts resolver
        resolver.set_hosts(get_static_hosts(local_hosts));

        Ok(Self {
            resolver,
            blacklist_controller,
            live_requests_controller,
            database_controller,
        })
    }

    async fn inner_handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        response_handle: &mut R,
    ) -> Result<ResponseInfo> {
        let query = request.query();
        let query_question = query.name().to_string();

        // TODO: Treat all questions !
        // TODO: Check trailing dots
        let mut rev_address = String::new();
        for component in query_question.trim_end_matches('.').split('.').rev() {
            rev_address.push_str(component);

            if let Ok(Some(domain_id)) = self
                .blacklist_controller
                .is_domain_blacklisted(&rev_address)
                .await
            {
                log::warn!("[{}] Blacklisted domain {}", request.src(), query_question);

                let response = dns_default_response(request, ResponseCode::Refused);
                let response_info = response_handle.send_response(response).await?;

                if let Err(e) = self
                    .add_blocked_request(request.src().ip(), domain_id)
                    .await
                {
                    log::error!("Error during database insertion: {}", e);
                }

                return Ok(response_info);
            }

            rev_address.push('.');
        }

        let response = self.resolver.lookup_ip(query.name().to_string()).await?;
        let response_builder = MessageResponseBuilder::from_message_request(request);
        let response = response.as_lookup();
        let mut response = response_builder.build(
            *request.header(),
            response.records(),
            vec![],
            vec![],
            request.additionals(),
        );

        response
            .header_mut()
            .set_message_type(MessageType::Response);

        let response_info = response_handle.send_response(response).await?;

        let client_address = request.src().ip().to_string();
        let client_id = self
            .database_controller
            .get_client_id(&client_address)
            .await?;

        self.live_requests_controller
            .notify(
                Some(LiveRequest {
                    request_id: request.id(),
                    client_address,
                    request_address: query_question,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64(),
                }),
                Some(client_id),
            )
            .await;

        Ok(response_info)
    }

    async fn add_blocked_request(&self, client_ip: IpAddr, domain_id: u32) -> Result<()> {
        // Insert it in database for future work
        let blocked_request = self
            .database_controller
            .add_blocked_request(client_ip, domain_id)
            .await?;

        // Notify all watchers that a new domain has been blocked
        self.blacklist_controller
            .notify_blocked(&blocked_request, blocked_request.client_id)
            .await;

        Ok(())
    }
}

#[async_trait::async_trait]
impl RequestHandler for RequestsController {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        match self
            .inner_handle_request(request, &mut response_handle)
            .await
        {
            Ok(r) => r,
            Err(_) => {
                let response = dns_default_response(request, ResponseCode::ServFail);
                response_handle
                    .send_response(response)
                    .await
                    .expect("Cannot send response to client")
            }
        }
    }
}
