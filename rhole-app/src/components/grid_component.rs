use yew::{function_component, html, Html, Properties};
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct GridComponentProps {
    pub name: String,
    pub icon: IconId,
    pub link_to: Route,
}

#[function_component]
pub fn GridComponent(props: &GridComponentProps) -> Html {
    html! {
        <Link<Route> to={props.link_to.to_owned()}>
            <div class="flex" style="border-radius: 25px; border: 2px solid black; padding: 15px; grid-column: auto;">
                <div>
                    <Icon height={"100px"} width={"100px"} icon_id={props.icon}/>
                </div>
                <h2>{props.name.to_owned()}</h2>
            </div>
        </Link<Route>>
    }
}
