use super::DatabaseController;
use crate::api_models::BlockedRequest;
use anyhow::anyhow;
use anyhow::Result;
use sea_orm::{ActiveValue, EntityTrait, QuerySelect};
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use entity::blocked_requests::ActiveModel as BlockedRequestActiveModel;
use entity::blocked_requests::Entity as BlockedRequestEntity;

impl DatabaseController {
    pub async fn add_blocked_request(
        &self,
        client_address: IpAddr,
        domain_id: i32,
    ) -> Result<BlockedRequest> {
        self.upsert_client(client_address).await?;
        self.increment_blocked_domain_counter(domain_id).await?;

        let client = self
            .get_client_from_addr(client_address.to_string())
            .await?
            .ok_or(anyhow!("Could not get client_id for {client_address}"))?;

        let blocked_request = BlockedRequestEntity::insert(BlockedRequestActiveModel {
            id: ActiveValue::NotSet,
            client_id: ActiveValue::Set(client.client_id),
            domain_id: ActiveValue::Set(domain_id),
            blocked_timestamp: ActiveValue::Set(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64(),
            ),
        })
        .exec_with_returning(&self.connection)
        .await?;

        let blocked_domain = self
            .get_blocked_domain(domain_id)
            .await?
            .ok_or(anyhow!("Could not get domain_id {domain_id}"))?;

        Ok(BlockedRequest::from((
            blocked_request,
            blocked_domain.domain_address,
        )))
    }

    pub async fn get_blocked_requests(&self, num: Option<u64>) -> Result<Vec<BlockedRequest>> {
        let blocked_requests = BlockedRequestEntity::find()
            .find_also_related(entity::blocked_domains::Entity)
            .limit(num)
            .all(&self.connection)
            .await?;

        Ok(blocked_requests
            .into_iter()
            .filter_map(|br| BlockedRequest::try_from(br).ok())
            .collect())
    }
}
