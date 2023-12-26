mod blocked_domains;
mod blocked_requests;
mod clients;
mod infos;

pub use blocked_domains::BlockedDomainsQuery;
pub use blocked_requests::BlockedRequestsQuery;
pub use clients::ClientsQuery;
pub use infos::InfosQuery;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct RholeQueries(
    ClientsQuery,
    InfosQuery,
    BlockedRequestsQuery,
    BlockedDomainsQuery,
);
