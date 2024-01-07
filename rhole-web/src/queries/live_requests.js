import { gql } from "@apollo/client";

export const LIVE_REQUESTS_SUBSCRIPTION = gql`subscription LiveRequests($clientId: Int) {
  liveRequests(clientId: $clientId) {
    requestId
    clientAddress
    requestAddress
    timestamp
    clientId
  }
}`;