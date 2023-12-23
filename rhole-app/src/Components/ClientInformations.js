import React from "react";
import { Box, Divider, Typography } from "@mui/material"
import { timestampToDate } from "../utils";
import { useSubscription } from "@apollo/client";
import { BLOCKED_REQUESTS_SUBSCRIPTION } from "../queries/blocked_requests";
import { LIVE_REQUESTS_SUBSCRIPTION } from "../queries/live_requests";
import RequestsDisplay from "./RequestsDisplay";

export const ClientInformations = (props) => {
    const { data: dataBlockedRequests, loading: loadingBlockedRequests } = useSubscription(BLOCKED_REQUESTS_SUBSCRIPTION, {
        variables: { clientId: props.client.clientId }
    });

    const { data: dataLiveRequests, loading: loadingLiveRequests } = useSubscription(LIVE_REQUESTS_SUBSCRIPTION, {
        variables: { clientId: props.client.clientId }
    });

    return (
        <Box
            display="flex"
            flexDirection="column"
            width="100%"
            height="100%"
        >
            <Box
                display="flex"
                flexDirection="row"
                flex={1}
                alignItems="center"
            >
                <Typography
                    flex={1}
                    sx={{ fontWeight: "bold", fontSize: 30 }}
                >{props.client.address}</Typography>
                <Typography
                    display="flex"
                    justifyContent="flex-end"
                    flex={1}
                    sx={{ fontSize: 15 }}
                >Last seen: {timestampToDate(props.client.lastSeen)}</Typography>
            </Box>
            <Divider />
            <Box
                display="flex"
                flexDirection="row"
                overflow="hidden"
                flex={9}
                sx={{
                    "& > div": {
                        width: "50%"
                    }
                }}
            >
                <RequestsDisplay
                    header="Live blocked requests"
                    data={dataBlockedRequests?.blockedRequests}
                    loading={loadingBlockedRequests}
                />
                <Divider orientation="vertical" />
                <RequestsDisplay
                    header="Live requests"
                    data={dataLiveRequests?.liveRequests}
                    loading={loadingLiveRequests}
                />
            </Box>
        </Box>
    )
}