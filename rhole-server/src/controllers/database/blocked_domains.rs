use entity::blocked_domains::ActiveModel as BlockedDomainActiveModel;
use entity::blocked_domains::Entity as BlockedDomainEntity;
use log::error;
use sea_orm::ActiveModelTrait;
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

    pub async fn get_blocked_domains(&self, num: Option<u64>) -> Result<Vec<BlockedDomain>> {
        let blocked_domains = BlockedDomainEntity::find()
            .limit(num)
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

    // pub async fn is_domain_blacklisted<S: AsRef<str>>(&self, domain: S) -> Result<Option<i32>> {
    //     let opt_domain = BlockedDomainEntity::find()
    //         .distinct()
    //         .filter(entity::blocked_domains::Column::DomainAddress.eq(domain.as_ref()))
    //         .filter(entity::blocked_domains::Column::Whitelisted.eq(false))
    //         .one(&self.connection)
    //         .await?;

    //     match opt_domain {
    //         Some(d) => Ok(Some(d.id)),
    //         None => Ok(None),
    //     }
    // }

    pub async fn init_blocked_domains(&self, blocked_domains: Vec<String>) -> Result<u64> {
        let mut entries_added = 0;

        for domain in blocked_domains {
            match self.insert_blocked_domain(domain).await {
                Ok(_) => entries_added += 1,
                Err(e) => log::error!("{}", e),
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

        BlockedDomainEntity::insert(domain)
            .do_nothing()
            .exec_without_returning(&self.connection)
            .await?;

        Ok(())
    }
}
