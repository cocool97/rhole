import { gql } from "@apollo/client";

export const BLOCKED_REQUESTS_QUERY = gql`query {
  blockedRequests {
    requestId
    clientId
    requestAddress
    timestamp
  }
}`;

export const BLOCKED_REQUESTS_SUBSCRIPTION = gql`subscription {
  blockedRequests {
    requestId
    clientId
    requestAddress
    timestamp
  }
}`;