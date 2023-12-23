import React, { useEffect } from "react";
import { useQuery } from "@apollo/client";
import { CLIENTS_QUERY } from "../queries/client";
import { Box, Divider, List, ListItemAvatar, ListItemButton, ListItemText, ListSubheader, Typography } from "@mui/material";
import PermIdentityIcon from '@mui/icons-material/PermIdentity';
import { ClientInformations } from "../Components/ClientInformations";
import { useSearchParams } from "react-router-dom";

const CLIENT_ID_PARAM_NAME = "client_id";

const Clients = () => {
    const { loading, error, data } = useQuery(CLIENTS_QUERY);
    const [searchParams, setSearchParams] = useSearchParams();
    const [currentClient, setCurrentClient] = React.useState(null);

    useEffect(() => {
        const queryClient = searchParams.get(CLIENT_ID_PARAM_NAME);
        if (queryClient && data) {
            const matches = data.clients.filter((client) => client.clientId === parseInt(queryClient));
            if (matches.length > 0) {
                setCurrentClient(matches[0])
            }
        }
    }, [data, searchParams]);

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
                            <Box
                                display="flex"
                                flexDirection="row"
                                sx={{
                                    "& > *": {
                                        display: "flex",
                                        flex: 1
                                    }
                                }}
                            >
                                <Typography justifyContent="start">Client(s) listing</Typography>
                                <Typography justifyContent="end">{data?.clients.length || 0} found</Typography>
                            </Box>
                        </ListSubheader>
                    }
                    sx={{
                        width: "100%",
                        overflow: "scroll"
                    }}
                >
                    {!loading && data?.clients.map((client) => {
                        return (
                            <ClientListItem
                                key={client.clientId}
                                client={client}
                                setCurrentClient={setCurrentClient}
                                setSearchParams={setSearchParams}
                            />
                        )
                    })}
                </List>
            </Box>
            <Box
                display="flex"
                flexDirection="column"
                flex={4}
                marginLeft={4}
            >
                {currentClient !== null &&
                    <ClientInformations client={currentClient} />}
            </Box>
        </Box>
    )
}

const ClientListItem = (props) => {
    return (
        <React.Fragment>
            <ListItemButton
                onClick={(_) => { props.setCurrentClient(props.client); props.setSearchParams({ [CLIENT_ID_PARAM_NAME]: props.client.clientId }) }}
            >
                <ListItemAvatar>
                    <PermIdentityIcon />
                </ListItemAvatar>
                <ListItemText
                    primary={props.client.address}
                />
            </ListItemButton>
            <Divider />
        </React.Fragment>
    )
}

export default Clients;