import { gql } from "@apollo/client";

export const BLOCKED_REQUESTS_QUERY = gql`query {
  blockedRequests {
    requestId
    clientId
    requestAddress
    timestamp
  }
}`;