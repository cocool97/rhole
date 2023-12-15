import React from "react";
import { Box } from "@mui/material"
import { timestampToDate } from "../utils";

export const ClientInformations = (props) => {
    return (
        <Box>
            <h1>{props.client.address}</h1>
            <h4>{timestampToDate(props.client.lastSeen)}</h4>
            <h4>{props.client.clientId}</h4>
        </Box>
    )
}