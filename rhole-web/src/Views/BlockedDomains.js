import React from "react";
import { useMutation, useQuery } from "@apollo/client";
import { DataGrid } from '@mui/x-data-grid';
import { BLOCKED_DOMAINS_QUERY, SET_WHITELIST_STATUS } from "../queries/blocked_domains";
import { timestampToDate } from "../utils";
import { Checkbox } from "@mui/material";

function RenderCheckBox(props) {
    const [checked, setChecked] = React.useState(props.value);
    const [setWhitelistStatus, { error }] = useMutation(SET_WHITELIST_STATUS);

    const handleChange = (event) => {
        const whitelisted = event.target.checked;
        setWhitelistStatus({ variables: { domainId: props.row.domainId, whitelisted: whitelisted } });
        if (error) { console.log(error); alert(error.message) }
        setChecked(whitelisted);
    };

    return (
        <Checkbox
            key={props.row.domainId}
            size="medium"
            checked={checked}
            onChange={handleChange}
        />
    );
}


const BlockedDomains = () => {
    const pageSizeOptions = [25, 50, 100];
    const [paginationModel, setPaginationModel] = React.useState({
        page: 0,
        pageSize: pageSizeOptions[0],
    });

    const { loading, error, data } = useQuery(BLOCKED_DOMAINS_QUERY, {
        variables: paginationModel
    });

    const pagedBlockedDomains = data?.pagedBlockedDomains;

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
            flex: 2
        },
        {
            field: 'insertTimestamp',
            headerName: "Insert timestamp",
            editable: false,
            sortable: true,
            flex: 2
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
            flex: 1,
            renderCell: RenderCheckBox
        }
    ]

    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <DataGrid
            initialState={{
                pagination: { paginationModel },
            }}
            disableSelectionOnClick
            disableRowSelectionOnClick
            loading={loading}
            rows={pagedBlockedDomains?.blockedDomains.map((x) => ({ ...x, insertTimestamp: timestampToDate(x.insertTimestamp) })) || []}
            getRowId={(row) => row.domainId}
            columns={columns}
            pageSizeOptions={pageSizeOptions}
            paginationMode="server"
            rowCount={pagedBlockedDomains?.totalRowCount || 0}
            onPaginationModelChange={setPaginationModel}
        />
    )
}

export default BlockedDomains;