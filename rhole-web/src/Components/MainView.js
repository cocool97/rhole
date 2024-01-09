import { Box } from "@mui/material";
import RholeAppBar from "./AppBar";
import React from "react";
import RholeDrawer from "./RholeDrawer";

const MainView = (props) => {
    const [open, setOpen] = React.useState(false);

    const handleDrawerClick = () => {
        setOpen(!open)
    }

    return (
        <Box sx={{ display: 'flex', maxHeight: '100vh', height: '100vh' }}>
            <RholeAppBar
                handleDrawerClick={handleDrawerClick}
            />
            <RholeDrawer open={open} />
            <Box component="main" sx={{ flexGrow: 1, p: 3, mt: "64px", "& > *": { height: "100%" } }}>
                {props.children}
            </Box>
        </Box>
    );
}

export default MainView;