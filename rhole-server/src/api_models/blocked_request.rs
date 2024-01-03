use anyhow::{anyhow, Ok};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::models::BlockedDomain;

#[derive(Clone, Deserialize, Serialize, PartialEq, SimpleObject)]
pub struct BlockedRequest {
    pub request_id: i32,
    pub client_id: i32,
    pub request_address: String,
    pub timestamp: f64,
}

impl From<(entity::blocked_requests::Model, String)> for BlockedRequest {
    fn from(value: (entity::blocked_requests::Model, String)) -> Self {
        Self {
            request_id: value.0.id,
            client_id: value.0.client_id,
            request_address: value.1,
            timestamp: value.0.blocked_timestamp,
        }
    }
}

impl
    TryFrom<(
        entity::blocked_requests::Model,
        Option<entity::blocked_domains::Model>,
    )> for BlockedRequest
{
    type Error = anyhow::Error;

    fn try_from(
        value: (
            entity::blocked_requests::Model,
            Option<entity::blocked_domains::Model>,
        ),
    ) -> Result<Self, Self::Error> {
        let blocked_request = value.0;
        let blocked_domain: BlockedDomain = value.1.ok_or(anyhow!("No domain related.."))?.into();

        Ok(Self {
            request_id: blocked_request.id,
            client_id: blocked_request.client_id,
            request_address: blocked_domain.domain_address,
            timestamp: blocked_request.blocked_timestamp,
        })
    }
}
