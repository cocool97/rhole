use std::{collections::HashMap, net::Ipv4Addr, str::FromStr};

use anyhow::Result;
use trust_dns_client::{
    op::{MessageType, Query, ResponseCode},
    rr::{DNSClass, RData, RecordType},
};
use trust_dns_resolver::{
    config::{NameServerConfig, ResolverConfig, ResolverOpts},
    lookup::Lookup,
    name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime},
    AsyncResolver, Hosts, Name,
};
use trust_dns_server::{
    authority::MessageResponseBuilder,
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};

use crate::{api_models::ProxyServer, models::dns_default_response};

use super::BlacklistController;

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

        hosts.insert(name, RecordType::A, Lookup::from_rdata(q, RData::A(*addr)));

        log::debug!("Adds static-host {host}: {addr}");
        nb += 1;
    }

    log::info!("Found {} static hosts...", nb);

    Some(hosts)
}

pub struct RequestsController {
    resolver: AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>,
    blacklist_controller: BlacklistController,
}

impl RequestsController {
    pub async fn new(
        proxy: ProxyServer,
        blacklist_controller: BlacklistController,
        local_hosts: &HashMap<String, Ipv4Addr>,
    ) -> Result<Self> {
        let name_server_config = NameServerConfig {
            socket_addr: std::net::SocketAddr::new(
                std::net::IpAddr::V4(proxy.ip.parse()?),
                proxy.port,
            ),
            protocol: trust_dns_resolver::config::Protocol::Tls,
            tls_dns_name: Some(proxy.tls_dns_name.clone()),
            trust_nx_responses: true,
            tls_config: None,
            bind_addr: None,
        };

        let mut resolver_config = ResolverConfig::new();
        resolver_config.add_name_server(name_server_config);

        let mut resolver = AsyncResolver::tokio(resolver_config, ResolverOpts::default())?;

        // Sets static hosts resolver
        resolver.set_hosts(get_static_hosts(local_hosts));

        Ok(Self {
            resolver,
            blacklist_controller,
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
                    .blacklist_controller
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

        Ok(response_handle.send_response(response).await?)
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
