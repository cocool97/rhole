mod blocked_requests;
mod live_requests;
pub use blocked_requests::BlockedRequestsSubscription;
pub use live_requests::LiveRequestsSubscription;

use async_graphql::MergedSubscription;

#[derive(MergedSubscription, Default)]
pub struct RholeSubscriptions(BlockedRequestsSubscription, LiveRequestsSubscription);
