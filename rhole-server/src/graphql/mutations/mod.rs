mod blocked_domains;

use blocked_domains::BlockedDomainsMutation;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct RholeMutations(BlockedDomainsMutation);
