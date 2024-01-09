import React from "react";
import { useQuery } from "@apollo/client";
import { Avatar, Box, Typography } from "@mui/material";
import SettingsIcon from '@mui/icons-material/Settings';
import TimerIcon from '@mui/icons-material/Timer';
import NumbersIcon from '@mui/icons-material/Numbers';
import DoNotDisturbOnTotalSilenceIcon from '@mui/icons-material/DoNotDisturbOnTotalSilence';
import { DASHBOARD_QUERY } from "../queries/server_infos";

const DASHBOARD_POLL_INTERVAL = 10;

export const Dashboard = () => {
    const { loading, data } = useQuery(DASHBOARD_QUERY, {
        pollInterval: DASHBOARD_POLL_INTERVAL * 1000
    });

    return (
        <Box
            display="flex"
            flexWrap="wrap"
            alignContent="baseline"
            columnGap="10px"
            rowGap="10px"
            justifyContent="space-around"
        >
            <MainPageElement
                loading={loading}
                data={
                    {
                        "name": "Uptime",
                        "value": data?.serverInfos.uptime,
                        "icon": <TimerIcon />
                    }} />
            <MainPageElement
                loading={loading}
                data={
                    {
                        "name": "Version",
                        "value": data?.serverInfos.buildVersion,
                        "icon": <SettingsIcon />
                    }
                } />
            <MainPageElement
                loading={loading}
                data={
                    {
                        "name": "Domains in blacklist",
                        "value": data?.blacklistInfos.count,
                        "icon": <NumbersIcon />
                    }} />
            <MainPageElement
                loading={loading}
                data={
                    {
                        "name": "Overall total blocked domains",
                        "value": data?.blacklistInfos.total,
                        "icon": <DoNotDisturbOnTotalSilenceIcon />
                    }
                } />
            <MainPageElement
                loading={loading}
                data={
                    {
                        "name": "Blacklist sources",
                        "value": data?.blacklistInfos.nbSources,
                        "icon": <NumbersIcon />
                    }
                } />
        </Box>
    )
}

const MainPageElement = (props) => {
    return (
        <Box
            display="flex"
            sx={{
                border: "1px black solid", borderRadius: "10px", "& > *": {
                    width: "100%"
                }
            }}
            padding="20px"
            flexDirection="row"
            height="70px"
            width="400px"
        >
            <Box
                display="flex"
                flexDirection="column"
                width="auto"
                justifyContent="center"
                alignItems="center"
            >
                <Avatar sx={{ marginBottom: "5px", bgcolor: "#1976d2" }}>{props.data.icon}</Avatar>
                <Typography fontSize={18} fontWeight="bold" sx={{ textAlign: "center", color: "#1976d2" }}>{props.data.name}</Typography>
            </Box>
            <Box
                display="flex"
                justifyContent="center"
                alignItems="center"
            >
                {<Typography sx={{ textAlign: "center" }}>{props.loading ? "Loading..." : props.data.value}</Typography>}
            </Box>
        </Box>
    )
}

MainPageElement.defaultProps = {
    title: ""
}