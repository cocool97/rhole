import React from "react";
import { useQuery } from "@apollo/client";
import { DataGrid } from '@mui/x-data-grid';
import { BLOCKED_DOMAINS_QUERY } from "../queries/blocked_domains";
import { timestampToDate } from "../utils";


const BlockedDomains = () => {
    const { loading, error, data } = useQuery(BLOCKED_DOMAINS_QUERY);

    const columns = [
        {
            field: 'domainId',
            headerName: "Domain ID",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'domainAddress',
            headerName: "Domain Address",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'insertTimestamp',
            headerName: "Insert timestamp",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'blockedCount',
            headerName: "Block count",
            editable: false,
            sortable: true,
            flex: 1
        },
        {
            field: 'whitelisted',
            headerName: "Whitelisted",
            editable: false,
            sortable: true,
            flex: 1
        }
    ]

    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <DataGrid
            initialState={{
                pagination: {
                    paginationModel: { pageSize: 25, page: 0 },
                },
            }}
            loading={loading}
            rows={data?.blockedDomains.map((x) => ({ ...x, insertTimestamp: timestampToDate(x.insertTimestamp) })) || []}
            getRowId={(row) => row.domainId}
            columns={columns}
            disableRowSelectionOnClick
        />
    )
}

export default BlockedDomains;