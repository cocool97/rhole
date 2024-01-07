import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import './App.css';
import Clients from "./Views/Clients";
import { ApolloClient, ApolloProvider, HttpLink, InMemoryCache, split } from "@apollo/client";
import { API_DOMAIN, API_ROUTE, API_SCHEME, WS_SCHEME } from "./Constants";
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import BlockedRequests from "./Views/BlockedRequests";
import BlockedDomains from "./Views/BlockedDomains";
import { getMainDefinition } from "@apollo/client/utilities";
import { createClient } from "graphql-ws";
import LiveBlockedRequests from "./Views/LiveRequests";
import MainView from "./Components/MainView";
import { Dashboard } from "./Views/MainPage";
import { GET_OWN_CLIENT_ID } from "./queries/client";

const App = () => {
    const [clientId, setClientId] = React.useState(null);
    const httpLink = new HttpLink({
        uri: API_SCHEME + API_DOMAIN + API_ROUTE + "/graphql"

    });

    const wsLink = new GraphQLWsLink(createClient({
        url: WS_SCHEME + API_DOMAIN + API_ROUTE + '/ws',
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

    apolloClient
        .query({
            query: GET_OWN_CLIENT_ID,
        })
        .then((result) => setClientId(result.data.getOwnClientId));

    return (
        <ApolloProvider client={apolloClient}>
            <BrowserRouter>
                <Routes>
                    <Route
                        path="/"
                        element={
                            <MainView>
                                <Dashboard />
                            </MainView>
                        }
                    />
                    <Route
                        path="/clients"
                        element={
                            <MainView>
                                <Clients
                                    ownClientId={clientId}
                                />
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
                                <BlockedRequests
                                    ownClientId={clientId}
                                />
                            </MainView>
                        }
                    />
                    <Route
                        path="/realtime"
                        element={
                            <MainView>
                                <LiveBlockedRequests
                                    ownClientId={clientId}
                                />
                            </MainView>
                        }
                    />
                </Routes>
            </BrowserRouter>
        </ApolloProvider>
    );
}

export default App;
