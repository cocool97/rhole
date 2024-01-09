mod blocked_domains;
mod clients;

use blocked_domains::BlockedDomainsMutation;
use clients::ClientsMutation;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct RholeMutations(BlockedDomainsMutation, ClientsMutation);
