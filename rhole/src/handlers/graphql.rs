use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;

use crate::models::RouterState;

pub async fn graphql(State(state): State<RouterState>, req: GraphQLRequest) -> GraphQLResponse {
    state
        .graphql_schema
        .execute(req.into_inner())
        .await
        .into()
}
