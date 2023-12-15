import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import './App.css';
import Clients from "./Views/Clients";
import { ApolloClient, ApolloProvider, HttpLink, InMemoryCache, split } from "@apollo/client";
import MainPage from "./Views/MainPage";
import MainView from "./Components/AppBar";
import { API_DOMAIN, API_SCHEME } from "./Constants";
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import BlockedRequests from "./Views/BlockedRequests";
import BlockedDomains from "./Views/BlockedDomains";
import ServerConfig from "./Views/ServerConfig";
import { getMainDefinition } from "@apollo/client/utilities";
import { createClient } from "graphql-ws";

const App = () => {
    const httpLink = new HttpLink({
        uri: API_SCHEME + API_DOMAIN + "/graphql"

    });

    const wsLink = new GraphQLWsLink(createClient({
        url: 'ws://' + API_DOMAIN + '/ws',
    }));

    // The split function takes three parameters:
    //
    // * A function that's called for each operation to execute
    // * The Link to use for an operation if the function returns a "truthy" value
    // * The Link to use for an operation if the function returns a "falsy" value
    const splitLink = split(
        ({ query }) => {
            const definition = getMainDefinition(query);
            return (
                definition.kind === 'OperationDefinition' &&
                definition.operation === 'subscription'
            );
        },
        wsLink,
        httpLink,
    );

    const apolloClient = new ApolloClient({
        link: splitLink,
        cache: new InMemoryCache(),
    });

    return (
        <ApolloProvider client={apolloClient}>
            <BrowserRouter>
                <Routes>
                    <Route
                        path="/"
                        element={
                            <MainView>
                                <MainPage />
                            </MainView>
                        }
                    />
                    <Route
                        path="/clients"
                        element={
                            <MainView>
                                <Clients />
                            </MainView>
                        }
                    />
                    <Route
                        path="/domains"
                        element={
                            <MainView>
                                <BlockedDomains />
                            </MainView>
                        }
                    />
                    <Route
                        path="/blocked"
                        element={
                            <MainView>
                                <BlockedRequests />
                            </MainView>
                        }
                    />
                    <Route
                        path="/realtime"
                        element={
                            <MainView>
                                TODO
                            </MainView>
                        }
                    />
                    <Route
                        path="/config"
                        element={
                            <MainView>
                                <ServerConfig />
                            </MainView>
                        }
                    />
                </Routes>
            </BrowserRouter>
        </ApolloProvider>
    );
}

export default App;
