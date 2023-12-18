import { gql } from "@apollo/client";

export const LIVE_REQUESTS_SUBSCRIPTION = gql`subscription {
  liveRequests {
    requestId
    clientAddress
    requestAddress
    timestamp
  }
}`;