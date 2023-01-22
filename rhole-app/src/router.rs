use yew::{function_component, html, Html};
use yew_router::BrowserRouter;
use yew_router::{Routable, Switch};

use crate::components::{App, BlockedRequests, Clients};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/index.html")]
    Index,
    #[at("/clients.html")]
    Clients,
    #[at("/blocked_requests.html")]
    BlockedRequests,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home | Route::Index => html! { <App /> },
        Route::Clients => html! {<Clients />},
        Route::BlockedRequests => html! {<BlockedRequests />},
        // TODO: 404 page !
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn RholeRouter() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
