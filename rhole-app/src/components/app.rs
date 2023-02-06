use yew::{function_component, html, Html};
use yew_icons::IconId;
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
                <GridComponent name={"Clients"} icon={IconId::BootstrapPeople} link_to={Route::Clients} />
                <GridComponent name={"Blocked Requests"} icon={IconId::BootstrapListCheck} link_to={Route::BlockedRequests} />
                <GridComponent name={"Stats"} icon={IconId::BootstrapBarChartLine} link_to={Route::Stats} />
                <GridComponent name={"Server informations"} icon={IconId::HeroiconsOutlineInformationCircle} link_to={Route::ServerInformations}/>
                <GridComponent name={"Server configuration"} icon={IconId::BootstrapFileEarmarkBinary} link_to={Route::Config} />
            </Grid>
        </BrowserRouter>
    }
}
