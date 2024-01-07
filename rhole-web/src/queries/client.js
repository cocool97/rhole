import { gql } from "@apollo/client";

export const CLIENTS_QUERY = gql`query {
    clients {
        clientId
        address
        lastSeen
    }
}`;

export const GET_OWN_CLIENT_ID = gql`
query {
    getOwnClientId
}`