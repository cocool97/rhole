import { gql } from "@apollo/client";

export const BLOCKED_DOMAINS_QUERY = gql`query {
  blockedDomains {
    domainId
    domainAddress
    insertTimestamp
    blockedCount
    whitelisted
  }
}`;