use std::net::SocketAddr;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{ConnectInfo, State};

use crate::models::RouterState;

pub async fn graphql(
    State(state): State<RouterState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state
        .graphql_schema
        .execute(req.into_inner().data(addr))
        .await
        .into()
}
