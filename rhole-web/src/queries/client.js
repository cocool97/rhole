import { gql } from "@apollo/client";

export const CLIENTS_QUERY = gql`query {
    clients {
        clientId
        address
        lastSeen
    }
}`;