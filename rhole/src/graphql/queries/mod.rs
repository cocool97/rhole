mod blocked_domains;
mod blocked_requests;
mod clients;
mod infos;
mod server_configuration;

pub use blocked_domains::BlockedDomainsQuery;
pub use blocked_requests::BlockedRequestsQuery;
pub use clients::ClientsQuery;
pub use infos::InfosQuery;
pub use server_configuration::ServerConfigurationQuery;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct RholeQueries(
    ClientsQuery,
    InfosQuery,
    BlockedRequestsQuery,
    BlockedDomainsQuery,
    ServerConfigurationQuery,
);
