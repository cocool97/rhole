import React from "react";
import { Box, Divider, TextField, Typography } from "@mui/material"
import { timestampToDate } from "../utils";
import { useMutation, useSubscription } from "@apollo/client";
import { BLOCKED_REQUESTS_SUBSCRIPTION } from "../queries/blocked_requests";
import { LIVE_REQUESTS_SUBSCRIPTION } from "../queries/live_requests";
import RequestsDisplay from "../Components/RequestsDisplay";
import { SET_CLIENT_ALIAS } from "../queries/client";

export const ClientInformations = (props) => {
    const { data: dataBlockedRequests, loading: loadingBlockedRequests } = useSubscription(BLOCKED_REQUESTS_SUBSCRIPTION, {
        variables: { clientId: props.client.clientId }
    });

    const { data: dataLiveRequests, loading: loadingLiveRequests } = useSubscription(LIVE_REQUESTS_SUBSCRIPTION, {
        variables: { clientId: props.client.clientId }
    });

    const [setClientAlias, { error }] = useMutation(SET_CLIENT_ALIAS);

    const remoteClientAliasUpdate = (event) => {
        setClientAlias({
            variables: {
                clientId: props.client.clientId,
                alias: event.target.value
            }
        });
        if (error) { console.log(error); alert(error.message) }
    }

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
                <TextField
                    id="standard-basic-controlled"
                    variant="standard"
                    size="medium"
                    defaultValue={props.client.alias ?? props.client.address}
                    sx={{
                        fontWeight: "bold",
                        fontSize: 30,
                        flex: 1,
                        mr: "50px"
                    }}
                    onBlur={(event) => remoteClientAliasUpdate(event)}
                    onKeyDown={(event) => {
                        // on Enter pressed
                        if (event.key === 13) {
                            remoteClientAliasUpdate(event);
                        }
                    }}
                />
                <Typography
                    justifyContent="flex-end"
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
                    key={props.client.clientId + "live"}
                    header="Live requests"
                    data={dataLiveRequests?.liveRequests}
                    loading={loadingLiveRequests}
                />
                <Divider orientation="vertical" />
                <RequestsDisplay
                    key={props.client.clientId + "live-blocked"}
                    header="Live blocked requests"
                    data={dataBlockedRequests?.blockedRequests}
                    loading={loadingBlockedRequests}
                />
            </Box>
        </Box>
    )
}