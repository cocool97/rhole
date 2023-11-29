import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import './App.css';
import Clients from "./Views/Clients";
import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import MainPage from "./Views/MainPage";
import MainView from "./Components/AppBar";
import { API_DOMAIN } from "./Constants";
import BlockedRequests from "./Views/BlockedRequests";
import BlockedDomains from "./Views/BlockedDomains";
import ServerConfig from "./Views/ServerConfig";

const App = () => {
    const apolloClient = new ApolloClient({
        uri: API_DOMAIN + "/graphql",
        cache: new InMemoryCache(),
        assumeImmutableResults: true
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
