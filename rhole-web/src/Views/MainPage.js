import React from "react";
import { useQuery } from "@apollo/client";
import { Box, List, ListItem, ListItemIcon, ListItemText, Typography } from "@mui/material";
import SettingsIcon from '@mui/icons-material/Settings';
import TimerIcon from '@mui/icons-material/Timer';
import NumbersIcon from '@mui/icons-material/Numbers';
import DoNotDisturbOnTotalSilenceIcon from '@mui/icons-material/DoNotDisturbOnTotalSilence';
import { DASHBOARD_QUERY } from "../queries/server_infos";

export const Dashboard = () => {
    const { loading, data } = useQuery(DASHBOARD_QUERY);

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
                        "value": data?.serverInfos.uptime,
                        "icon": <TimerIcon />
                    },
                    {
                        "name": "Version",
                        "value": data?.serverInfos.buildVersion,
                        "icon": <SettingsIcon />
                    }
                ]} />
            <MainPageElement
                title="Blacklist statistics"
                loading={loading}
                data={[
                    {
                        "name": "Domains in blacklist",
                        "value": data?.blacklistInfos.count,
                        "icon": <NumbersIcon />
                    },
                    {
                        "name": "Overall total blocked domains",
                        "value": data?.blacklistInfos.total,
                        "icon": <DoNotDisturbOnTotalSilenceIcon />
                    },
                    {
                        "name": "Blacklist sources",
                        "value": data?.blacklistInfos.nbSources,
                        "icon": <NumbersIcon />
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
            height="fit-content"
        >
            <Typography textAlign="center" height="fit-content" component="h5" fontWeight="bold" fontSize={20}>{props.title}</Typography>
            <Box>
                <List>
                    {props.data.map((element, index) => {
                        return (<ListItem key={index}>
                            <ListItemIcon>
                                {element.icon}
                            </ListItemIcon>
                            <ListItemText primary={element.name} />
                            {props.loading ? <Typography textAlign={"end"}>Loading...</Typography> : <ListItemText primary={element.value} sx={{ textAlign: "end" }} />}
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