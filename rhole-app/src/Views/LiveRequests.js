import React, { useEffect } from "react";
import { useSubscription } from "@apollo/client";
import { DataGrid } from '@mui/x-data-grid';
import { timestampToDate } from "../utils";
import { LIVE_REQUESTS_SUBSCRIPTION } from "../queries/live_requests";

function LiveBlockedRequestsDisplay({ loading, data }) {
    const [liveRequests, setLiveRequests] = React.useState([]);

    useEffect(() => {
        if (data) {
            const liveRequests = data.liveRequests;
            setLiveRequests((oldState) => [...oldState, { ...liveRequests, timestamp: timestampToDate(liveRequests.timestamp) }]);
        }
    }, [data])

    const columns = [
        {
            field: 'requestAddress',
            headerName: "Request Address",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'clientAddress',
            headerName: "Client Address",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'timestamp',
            headerName: "Date",
            editable: false,
            sortable: true,
            flex: 1
        }
    ];

    return (
        <DataGrid
            loading={loading}
            rows={liveRequests}
            getRowId={(row) => row.requestId}
            columns={columns}
            autoPageSize
            disableRowSelectionOnClick
        />
    )
}

const LiveBlockedRequests = () => {
    const { data, loading } = useSubscription(LIVE_REQUESTS_SUBSCRIPTION, {
        variables: { clientId: null }
    });

    return (
        <LiveBlockedRequestsDisplay
            loading={loading}
            data={data}
        />
    )
}

export default LiveBlockedRequests;