import { Box } from "@mui/material";
import React from "react";
import StarIcon from '@mui/icons-material/Star';


export function RenderOwnIdCell(value, clientId, ownClientId) {
    return (
        <>
            <Box flex={1}>{value}</Box>
            {clientId === ownClientId && <StarIcon />}
        </>
    );
}