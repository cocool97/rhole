import React from "react";
import { useQuery } from "@apollo/client";
import { CLIENTS_QUERY } from "../queries/client";
import { List, ListItem, ListItemAvatar, ListItemText, ListSubheader } from "@mui/material";
import FolderIcon from '@mui/icons-material/Folder';

const Clients = () => {
    const { loading, error, data } = useQuery(CLIENTS_QUERY);

    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <List
            subheader={
                <ListSubheader>
                    All clients
                </ListSubheader>
            }
        >
            {!loading && data?.clients.map((client) => {
                return (<ListItem>
                    <ListItemAvatar>
                        <FolderIcon />
                    </ListItemAvatar>
                    <ListItemText
                        primary={client.clientId}
                        secondary={client.address}
                    />
                </ListItem>)
            })}
        </List>

    )
}

export default Clients;