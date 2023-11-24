import React from "react";
import { useQuery } from "@apollo/client";
import { SERVER_CONFIG_QUERY } from "../queries/server_config";


const ServerConfig = () => {
    const { error, data } = useQuery(SERVER_CONFIG_QUERY);

    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <pre>
            {JSON.stringify(data, null, 2)}
        </pre>
    )
}

export default ServerConfig;