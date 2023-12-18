import React from "react";
import { Box, Divider, Typography } from "@mui/material"
import { timestampToDate } from "../utils";

export const ClientInformations = (props) => {
    return (
        <Box
            display="flex"
            flexDirection="column"
            width="100%"
            height="100%"
            sx={{
                "& > *": {
                    flex: 1
                }
            }}
        >
            <Box>
                <h1>{props.client.address}</h1>
                <h4>{timestampToDate(props.client.lastSeen)}</h4>
                <h4>{props.client.clientId}</h4>
            </Box>
            <Box
                display="flex"
                flexDirection="row"
            >
                <Box
                    flex={1}
                >
                    <Typography align="center">Real time requests</Typography>
                </Box>
                <Divider orientation="vertical" />
                <Box
                    flex={1}
                >
                    <Typography align="center">Blocked requests</Typography>
                </Box>
            </Box>
        </Box>
    )
}