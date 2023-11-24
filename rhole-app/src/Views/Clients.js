import React from "react";
import { useQuery } from "@apollo/client";
import { DataGrid } from '@mui/x-data-grid';
import { CLIENTS_QUERY } from "../queries/client";


const Clients = () => {
    const { loading, error, data } = useQuery(CLIENTS_QUERY);

    const columns = [
        {
            field: 'clientId',
            headerName: "Client ID",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'address',
            headerName: "Address",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'lastSeen',
            headerName: "Last seen",
            editable: false,
            sortable: true,
            flex: 1
        }
    ]

    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <DataGrid
            loading={loading}
            rows={data?.clients || []}
            getRowId={(row) => row.clientId}
            columns={columns}
            autoPageSize
            disableRowSelectionOnClick
        />
    )
}

export default Clients;