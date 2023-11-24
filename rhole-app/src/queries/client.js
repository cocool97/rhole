import { gql } from "@apollo/client";

export const CLIENTS_QUERY = gql`query {
    clients {
        clientId
        address
        lastSeen
    }
}`;

export const INFOS_QUERY = gql`query {
    infos {
        uptime
        buildVersion
        buildTimestamp
        buildOsVersion
    }
}`;