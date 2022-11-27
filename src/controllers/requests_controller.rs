use anyhow::Result;
use sled::Db;
use trust_dns_client::op::{MessageType, ResponseCode};
use trust_dns_resolver::{
    config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts},
    name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime},
    AsyncResolver,
};
use trust_dns_server::{
    authority::MessageResponseBuilder,
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};

use crate::models::{dns_default_response, ProxyServer};

pub struct RequestsController {
    pub blacklist: Db,
    resolver: AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>,
}

impl RequestsController {
    pub async fn new(blacklist: Db, proxy: ProxyServer) -> Result<Self> {
        let mut resolver_config = ResolverConfig::default();
        resolver_config.add_name_server(NameServerConfig::new(proxy.try_into()?, Protocol::Udp));
        let resolver = AsyncResolver::tokio(resolver_config, ResolverOpts::default())?;

        Ok(Self {
            blacklist,
            resolver,
        })
    }

    async fn inner_handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        response_handle: &mut R,
    ) -> Result<ResponseInfo> {
        let query = request.query();
        let query_question = query.name().to_string();
        let query_type = query.query_type();

        // TODO: Treat all questions !
        // TODO: Check trailing dots
        let mut rev_address = String::new();
        for component in query_question.trim_end_matches('.').split('.').rev() {
            rev_address.push_str(component);

            if let Ok(Some(_)) = self.blacklist.get(&rev_address) {
                log::warn!("[{}] Blacklisted domain {}", request.src(), query_question);

                let response = dns_default_response(request, ResponseCode::Refused);

                return Ok(response_handle.send_response(response).await?);
            }

            rev_address.push('.');
        }

        let response = self.resolver.lookup(query.name(), query_type).await?;

        let response_builder = MessageResponseBuilder::from_message_request(request);
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
