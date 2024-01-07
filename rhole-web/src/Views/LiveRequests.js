import React, { useEffect } from "react";
import { useSubscription } from "@apollo/client";
import { DataGrid } from '@mui/x-data-grid';
import { timestampToDate } from "../utils";
import { LIVE_REQUESTS_SUBSCRIPTION } from "../queries/live_requests";
import { RenderOwnIdCell } from "../Components/RenderOwnIdCell";

function LiveBlockedRequestsDisplay({ ownClientId, loading, data }) {
    const [liveRequests, setLiveRequests] = React.useState([]);

    useEffect(() => {
        if (data) {
            const liveRequests = data.liveRequests;
            setLiveRequests((oldState) => [{ ...liveRequests, timestamp: timestampToDate(liveRequests.timestamp) }, ...oldState]);
        }
    }, [data])

    const columns = [
        {
            field: 'requestAddress',
            headerName: "Request Address",
            editable: false,
            sortable: true,
            flex: 2,
        },
        {
            field: "clientId",
            headerName: "Client ID",
            editable: false,
            sortable: true,
            flex: 1,
            renderCell: (props) => RenderOwnIdCell(props.row.clientId, props.row.clientId, ownClientId)
        },
        {
            field: 'clientAddress',
            headerName: "Client Address",
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

const LiveBlockedRequests = (props) => {
    const { data, loading } = useSubscription(LIVE_REQUESTS_SUBSCRIPTION, {
        variables: { clientId: null }
    });

    return (
        <LiveBlockedRequestsDisplay
            ownClientId={props.ownClientId}
            loading={loading}
            data={data}
        />
    )
}

export default LiveBlockedRequests;