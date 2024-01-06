mod blocked_domains;
mod blocked_requests;
mod clients;
mod server_infos;

pub use blocked_domains::BlockedDomainsQuery;
pub use blocked_requests::BlockedRequestsQuery;
pub use clients::ClientsQuery;
pub use server_infos::ServerInfosQuery;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct RholeQueries(
    ClientsQuery,
    ServerInfosQuery,
    BlockedRequestsQuery,
    BlockedDomainsQuery,
);
