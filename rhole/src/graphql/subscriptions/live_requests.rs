use crate::{models::GraphQLState, api_models::LiveRequest};
use async_graphql::{Context, Subscription};
use tokio_stream::{wrappers::WatchStream, Stream};

#[derive(Default)]
pub struct LiveRequestsSubscription;

#[Subscription]
impl LiveRequestsSubscription {
    async fn live_requests<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> impl Stream<Item = Option<LiveRequest>> {
        let receiver = ctx
            .data_unchecked::<GraphQLState>()
            .live_requests_controller
            .add_watcher()
            .await;

        WatchStream::from_changes(receiver)
    }
}
