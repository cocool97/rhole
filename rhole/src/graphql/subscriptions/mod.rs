mod blocked_requests;

pub use blocked_requests::BlockedRequestsSubscription;

use async_graphql::MergedSubscription;

#[derive(MergedSubscription, Default)]
pub struct RholeSubscriptions(BlockedRequestsSubscription);
