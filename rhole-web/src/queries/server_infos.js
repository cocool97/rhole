import { gql } from "@apollo/client";

export const DASHBOARD_QUERY = gql`query {
    serverInfos {
        uptime
        buildVersion
    }

    blacklistInfos {
      total
      count
      nbSources
    }
}`;