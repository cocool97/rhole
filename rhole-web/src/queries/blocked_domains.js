import { gql } from "@apollo/client";

export const BLOCKED_DOMAINS_QUERY = gql`
query BlockedDomains($page: Int, $pageSize: Int) {
  pagedBlockedDomains(page: $page, pageSize: $pageSize) {
    blockedDomains {
      domainId
      domainAddress
      insertTimestamp
      blockedCount
      whitelisted
    }
    totalRowCount
  }
}`;

export const SET_WHITELIST_STATUS = gql`
  mutation SetDomainWhitelistStatus($domainId: Int, $whitelisted: Boolean) {
    setDomainWhitelistStatus(domainId: $domainId, whitelisted: $whitelisted)
  }
`;