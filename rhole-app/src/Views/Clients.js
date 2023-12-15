import React from "react";
import { useQuery } from "@apollo/client";
import { CLIENTS_QUERY } from "../queries/client";
import { Box, Divider, List, ListItemAvatar, ListItemButton, ListItemText, ListSubheader } from "@mui/material";
import PermIdentityIcon from '@mui/icons-material/PermIdentity';
import { ClientInformations } from "../Components/ClientInformations";

const Clients = () => {
    const { loading, error, data } = useQuery(CLIENTS_QUERY);
    const [currentClient, setCurrentClient] = React.useState(null);

    if (error) { console.log(error); return <>{error.message}</> }

    return (
        <Box
            display="flex"
            flexDirection="row"
        >
            <Box
                display="flex"
                flex={1}
                marginRight={4}
            >
                <List
                    subheader={
                        <ListSubheader>
                            Client listing
                        </ListSubheader>
                    }
                    sx={{
                        width: "100%",
                        overflow: "scroll"
                    }}
                >
                    {!loading && data?.clients.map((client) => {
                        return (
                            <React.Fragment
                                key={client.clientId}
                            >
                                <ListItemButton
                                    onClick={(_) => setCurrentClient(client)}
                                >
                                    <ListItemAvatar>
                                        <PermIdentityIcon />
                                    </ListItemAvatar>
                                    <ListItemText
                                        primary={client.address}
                                    />
                                </ListItemButton>
                                <Divider />
                            </React.Fragment>
                        )
                    })}
                </List>
            </Box>
            <Box
                display="flex"
                flexDirection="column"
                flex={3}
                marginLeft={4}
            >
                {currentClient !== null &&
                    <ClientInformations client={currentClient} />}
            </Box>
        </Box>
    )
}

export default Clients;