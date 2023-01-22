use yew::{function_component, html, Html};
use yew_router::BrowserRouter;

use crate::{
    components::{Grid, GridComponent, Title},
    router::Route,
};

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Title />
            <Grid>
                // <Clients />
                // <BlockedRequests />
                <GridComponent name={"Clients"} icon="assets/user.svg" link_to={Route::Clients} />
                <GridComponent name={"Blocked Requests"} icon="assets/list.svg" link_to={Route::BlockedRequests} />
            </Grid>
        </BrowserRouter>
    }
}
