import { gql } from "@apollo/client";

export const SERVER_CONFIG_QUERY = gql`query {
  serverConfiguration {
   databasePath
    tls {
      certificatePath
      pkeyPath
    }
    webResources {
      staticFiles
      mountPath
      indexFile
    }
    localHosts
    net {
      dns {
        listenAddr
        listenPort
      }
      dot {
        listenAddr
        listenPort
        timeout
      }
      webInterface {
        listenAddr
        listenPort
      }
    }
    proxyServer {
      ip
      port
      tlsDnsName
    }
    sources {
      updateInterval
      entries {
        sourceType
        location
        comment
      }
    }

  }
}`;