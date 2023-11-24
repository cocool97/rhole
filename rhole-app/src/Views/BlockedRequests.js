import React from "react";
import { useQuery } from "@apollo/client";
import { DataGrid } from '@mui/x-data-grid';
import { BLOCKED_REQUESTS_QUERY } from "../queries/blocked_requests";


const BlockedRequests = () => {
    const { loading, error, data } = useQuery(BLOCKED_REQUESTS_QUERY);

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
            flex: 1
        },
        {
            field: 'requestAddress',
            headerName: "Request Address",
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
    ]

    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <DataGrid
            loading={loading}
            rows={data?.blockedRequests || []}
            getRowId={(row) => row.requestId}
            columns={columns}
            autoPageSize
            disableRowSelectionOnClick
        />
    )
}

export default BlockedRequests;