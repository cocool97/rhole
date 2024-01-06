use anyhow::anyhow;
use entity::blocked_domains::ActiveModel as BlockedDomainActiveModel;
use entity::blocked_domains::Column as BlockedDomainColumn;
use entity::blocked_domains::Entity as BlockedDomainEntity;
use log::error;
use sea_orm::ActiveModelTrait;
use sea_orm::FromQueryResult;
use sea_orm::PaginatorTrait;
use sea_orm::TransactionTrait;
use std::time::{SystemTime, UNIX_EPOCH};

use super::DatabaseController;
use crate::models::BlockedDomain;
use anyhow::Result;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

impl DatabaseController {
    pub async fn increment_blocked_domain_counter(&self, domain_id: i32) -> Result<()> {
        match BlockedDomainEntity::find_by_id(domain_id)
            .one(&self.connection)
            .await?
        {
            Some(blocked_domain) => {
                let blocked_count = blocked_domain.blocked_count;
                let mut model: BlockedDomainActiveModel = blocked_domain.into();
                model.blocked_count = ActiveValue::Set(blocked_count + 1);
                model.update(&self.connection).await?;
            }
            None => {
                error!("Blocked domain not found for id {}", domain_id);
            }
        }

        Ok(())
    }

    pub async fn get_blocked_domains(
        &self,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<BlockedDomain>> {
        let blocked_domains = BlockedDomainEntity::find()
            .filter(BlockedDomainColumn::Id.between(1 + page * page_size, (page + 1) * page_size))
            .all(&self.connection)
            .await?;

        Ok(blocked_domains.into_iter().map(Into::into).collect())
    }

    pub async fn get_blocked_domain(&self, domain_id: i32) -> Result<Option<BlockedDomain>> {
        let opt_blocked_domain = BlockedDomainEntity::find_by_id(domain_id)
            .one(&self.connection)
            .await?;

        Ok(opt_blocked_domain.map(Into::into))
    }

    pub async fn is_domain_blacklisted<I>(&self, rev_domain: I) -> Result<Option<i32>>
    where
        I: IntoIterator<Item = String>,
    {
        let opt_domain = BlockedDomainEntity::find()
            .distinct()
            .filter(entity::blocked_domains::Column::DomainAddress.is_in(rev_domain))
            .filter(entity::blocked_domains::Column::Whitelisted.eq(false))
            .one(&self.connection)
            .await?;

        match opt_domain {
            Some(d) => Ok(Some(d.id)),
            None => Ok(None),
        }
    }

    pub async fn init_blocked_domains(&self, blocked_domains: Vec<String>) -> Result<u64> {
        let mut entries_added = 0;

        for domain in blocked_domains {
            if self.insert_blocked_domain(domain).await.is_ok() {
                entries_added += 1;
            }
        }

        Ok(entries_added)
    }

    pub async fn insert_blocked_domain(&self, domain_address: String) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let domain = entity::blocked_domains::ActiveModel {
            id: ActiveValue::NotSet,
            domain_address: ActiveValue::Set(domain_address),
            insert_timestamp: ActiveValue::Set(timestamp),
            blocked_count: ActiveValue::Set(0),
            whitelisted: ActiveValue::Set(false),
        };

        // Execute it in a transaction as it'll rollback if an error occurred
        // (and in particularly if already existing)
        self.connection
            .transaction(move |db| {
                Box::pin(BlockedDomainEntity::insert(domain).exec_without_returning(db))
            })
            .await?;

        Ok(())
    }

    pub async fn set_domain_whitelist_status(
        &self,
        domain_id: i32,
        whitelisted: bool,
    ) -> Result<()> {
        let domain = BlockedDomainActiveModel {
            id: ActiveValue::Unchanged(domain_id),
            domain_address: ActiveValue::NotSet,
            insert_timestamp: ActiveValue::NotSet,
            blocked_count: ActiveValue::NotSet,
            whitelisted: ActiveValue::Set(whitelisted),
        };

        BlockedDomainEntity::update(domain)
            .exec(&self.connection)
            .await?;

        Ok(())
    }

    pub async fn get_blocked_domains_entries_count(&self) -> Result<u64> {
        Ok(BlockedDomainEntity::find()
            .distinct()
            .count(&self.connection)
            .await?)
    }

    pub async fn get_blocked_domains_sum(&self) -> Result<i64> {
        #[derive(FromQueryResult)]
        struct Sum {
            sum: i64,
        }

        let a = BlockedDomainEntity::find()
            .select_only()
            .column_as(BlockedDomainColumn::BlockedCount.sum(), "sum")
            .into_model::<Sum>()
            .one(&self.connection)
            .await?;

        Ok(a.ok_or(anyhow!("Could not get sum of blocked domains"))?
            .sum)
    }
}
