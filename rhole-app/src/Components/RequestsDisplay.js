import { Divider, Typography, List, Box, ListItem, ListItemText } from "@mui/material";
import React, { useEffect, useState } from "react";
import { timestampToDate } from "../utils";

const RequestsDisplay = (props) => {
    const [requests, setRequests] = useState([]);

    useEffect(() => {
        if (props.data) {
            setRequests((oldState) => [{ ...props.data, timestamp: timestampToDate(props.data.timestamp) }, ...oldState]);
        }
    }, [props.data])

    return (
        <Box
            height="100%"
            display="flex"
            flexDirection="column"
        >
            <Typography align="center" m={2} fontWeight="bold">{props.header}</Typography>
            <Divider />
            <Box
                overflow="scroll"
            >
                <List
                    sx={{ width: "100%", height: "100%" }}
                >
                    {requests.map((value) => {
                        return (
                            <ListItem key={value.requestId}>
                                <ListItemText sx={{ textAlign: "start" }}>{value.requestAddress}</ListItemText>
                                <ListItemText sx={{ textAlign: "end" }}>{value.timestamp}</ListItemText>
                            </ListItem>
                        )
                    })}
                </List>
            </Box>
        </Box>
    )
}

export default RequestsDisplay;