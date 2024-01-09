import { gql } from "@apollo/client";

export const CLIENTS_QUERY = gql`query {
    clients {
        clientId
        address
        alias
        lastSeen
    }
}`;

export const SET_CLIENT_ALIAS = gql`
  mutation SetClientAlias($clientId: Int, $alias: String) {
    setClientAlias(clientId: $clientId, alias: $alias)
  }
`;