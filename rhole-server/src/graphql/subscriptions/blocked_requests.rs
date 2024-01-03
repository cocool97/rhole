use crate::{api_models::BlockedRequest, models::GraphQLState};
use async_graphql::{Context, Subscription};
use tokio_stream::{wrappers::WatchStream, Stream};

#[derive(Default)]
pub struct BlockedRequestsSubscription;

#[Subscription]
impl BlockedRequestsSubscription {
    async fn blocked_requests<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        client_id: Option<i32>,
    ) -> impl Stream<Item = Option<BlockedRequest>> {
        let receiver = ctx
            .data_unchecked::<GraphQLState>()
            .blocked_requests_controller
            .add_watcher(client_id)
            .await;

        WatchStream::from_changes(receiver)
    }
}
