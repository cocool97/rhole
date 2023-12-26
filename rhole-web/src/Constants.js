
export const API_SCHEME = (!process.env.NODE_ENV || process.env.NODE_ENV === 'development') ? "http://" : "https://";
export const API_DOMAIN = (!process.env.NODE_ENV || process.env.NODE_ENV === 'development') ? "localhost:40443" : "rhole.lan";
export const API_ROUTE = "/api";
export const WS_SCHEME = (!process.env.NODE_ENV || process.env.NODE_ENV === 'development') ? "ws://" : "wss://";