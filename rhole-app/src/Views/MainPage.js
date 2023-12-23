import React from "react";
import { useQuery } from "@apollo/client";
import { Box, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Typography } from "@mui/material";
import { INFOS_QUERY } from "../queries/client";
import InboxIcon from '@mui/icons-material/Inbox';

const Dashboard = () => {
    const { loading, data } = useQuery(INFOS_QUERY);

    return (
        <Box
            display="grid"
            gridTemplateColumns="repeat(3, 1fr)"
            rowGap="10px"
            columnGap="10px"
        >
            <MainPageElement
                title="Server informations"
                loading={loading}
                data={[
                    {
                        "name": "Uptime",
                        "value": data?.infos.uptime
                    },
                    {
                        "name": "Version",
                        "value": data?.infos.buildVersion
                    }
                ]} />
        </Box>
    )
}

const MainPageElement = (props) => {
    return (
        <Box
            display="flex"
            sx={{
                border: "1px black solid", borderRadius: "30px", "& > *": {
                    width: "100%"
                }
            }}
            padding="20px"
            flexDirection="column"
        >
            <Typography textAlign="center" height="fit-content" component="h6" fontWeight="bold">{props.title}</Typography>
            <Box>
                <List>
                    {props.data.map((element, index) => {
                        return (<ListItem key={index} disablePadding>
                            <ListItemButton>
                                <ListItemIcon>
                                    <InboxIcon />
                                </ListItemIcon>
                                <ListItemText primary={element.name} />
                                {props.loading ? <Typography>Loading...</Typography> : <ListItemText primary={element.value} />}
                            </ListItemButton>
                        </ListItem>)
                    })}
                </List>
            </Box>
        </Box>
    )
}

MainPageElement.defaultProps = {
    title: ""
}

export default Dashboard;