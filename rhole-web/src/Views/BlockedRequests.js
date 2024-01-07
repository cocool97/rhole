import React, { useEffect } from "react";
import { useQuery } from "@apollo/client";
import { DataGrid } from '@mui/x-data-grid';
import { BLOCKED_REQUESTS_QUERY, BLOCKED_REQUESTS_SUBSCRIPTION } from "../queries/blocked_requests";
import { timestampToDate } from "../utils";
import { RenderOwnIdCell } from "../Components/RenderOwnIdCell";

function BlockedRequestsDisplay({ ownClientId, loading, updateCallback, data }) {
    const columns = [
        {
            field: 'requestId',
            headerName: "Request ID",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'clientId',
            headerName: "Client ID",
            editable: false,
            sortable: true,
            flex: 1,
            renderCell: (props) => RenderOwnIdCell(props.row.clientId, props.row.clientId, ownClientId)
        },
        {
            field: 'requestAddress',
            headerName: "Request Address",
            editable: false,
            sortable: true,
            flex: 2
        },
        {
            field: 'timestamp',
            headerName: "Date",
            editable: false,
            sortable: true,
            flex: 2
        }
    ];

    useEffect(() => updateCallback(), [updateCallback]);

    return (
        <DataGrid
            loading={loading}
            rows={data?.blockedRequests?.map((x) => ({
                ...x, timestamp: timestampToDate(x.timestamp)
            })) || []}
            getRowId={(row) => row.requestId}
            columns={columns}
            autoPageSize
            disableRowSelectionOnClick
        />
    )
}

const BlockedRequests = (props) => {
    const { subscribeToMore, loading, error, data } = useQuery(BLOCKED_REQUESTS_QUERY);

    const updateCallback = subscribeToMore({
        document: BLOCKED_REQUESTS_SUBSCRIPTION,
        variables: {},
        updateQuery: (prev, { subscriptionData }) => {
            if (!subscriptionData.data || subscriptionData.data.blockedRequests === null) return prev;
            const blockedRequest = subscriptionData.data.blockedRequests;

            const first = prev.blockedRequests[0];
            if (first === null || first.requestId === blockedRequest.requestId) {
                return prev
            }

            return {
                blockedRequests: [blockedRequest, ...prev.blockedRequests]
            }
        }
    })



    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <BlockedRequestsDisplay
            ownClientId={props.ownClientId}
            loading={loading}
            updateCallback={updateCallback}
            data={data}
        />
    )
}

export default BlockedRequests;