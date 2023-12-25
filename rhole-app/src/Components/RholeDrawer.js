import React from "react";
import { Divider, List, ListItem, ListItemButton, ListItemIcon, ListItemText, styled } from "@mui/material";
import { Link } from "react-router-dom";
import MuiDrawer from '@mui/material/Drawer';
import PeopleIcon from '@mui/icons-material/People';
import DangerousIcon from '@mui/icons-material/Dangerous';
import ChecklistIcon from '@mui/icons-material/Checklist';
import HomeIcon from '@mui/icons-material/Home';
import WatchLaterIcon from '@mui/icons-material/WatchLater';
import { DrawerHeader } from "./DrawerHeader";

const drawerWidth = 240;

const drawerItems = [
    {
        "name": "Dashboard",
        "logo": <HomeIcon />,
        "redirectTo": "/"
    },
    {
        "name": "Clients",
        "logo": <PeopleIcon />,
        "redirectTo": "/clients"
    },
    {
        "name": "Blocked requests",
        "logo": <DangerousIcon />,
        "redirectTo": "/blocked"
    },
    {
        "name": "Blocking list",
        "logo": <ChecklistIcon />,
        "redirectTo": "/domains"
    },
    {
        "name": "Real-time traffic",
        "logo": <WatchLaterIcon />,
        "redirectTo": "/realtime"
    },
];

const Drawer = styled(MuiDrawer, { shouldForwardProp: (prop) => prop !== 'open' })(
    ({ theme, open }) => ({
        width: drawerWidth,
        flexShrink: 0,
        whiteSpace: 'nowrap',
        boxSizing: 'border-box',
        ...(open && {
            ...openedMixin(theme),
            '& .MuiDrawer-paper': openedMixin(theme),
        }),
        ...(!open && {
            ...closedMixin(theme),
            '& .MuiDrawer-paper': closedMixin(theme),
        }),
    }),
);

const openedMixin = (theme) => ({
    width: drawerWidth,
    transition: theme.transitions.create('width', {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.enteringScreen,
    }),
    overflowX: 'hidden',
});

const closedMixin = (theme) => ({
    transition: theme.transitions.create('width', {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.leavingScreen,
    }),
    overflowX: 'hidden',
    width: `calc(${theme.spacing(7)} + 1px)`,
    [theme.breakpoints.up('sm')]: {
        width: `calc(${theme.spacing(8)} + 1px)`,
    },
});

const RholeDrawer = (props) => {
    return (
        <Drawer variant="permanent" open={props.open}>
            <DrawerHeader />
            <Divider />
            <List>
                {drawerItems.map((elem) => (
                    <Link key={elem.name} to={elem.redirectTo} title={elem.name} style={{ color: 'inherit', textDecoration: 'inherit' }}>
                        <ListItem key={elem.name} disablePadding sx={{ display: 'block' }}>
                            <ListItemButton
                                sx={{
                                    minHeight: 48,
                                    justifyContent: props.open ? 'initial' : 'center',
                                    px: 2.5,
                                }}
                            >
                                <ListItemIcon
                                    sx={{
                                        minWidth: 0,
                                        mr: props.open ? 3 : 'auto',
                                        justifyContent: 'center',
                                    }}
                                >
                                    {elem.logo}
                                </ListItemIcon>
                                <ListItemText primary={elem.name} sx={{ opacity: props.open ? 1 : 0 }} />
                            </ListItemButton>
                        </ListItem>
                    </Link>
                ))}
            </List>
        </Drawer>
    )
}

export default RholeDrawer;