use yew::{function_component, html, Html};

use crate::components::{Card, Clients, Title};

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <Title />
            <div>
                <Card />
                <Card />
                <Card />
                <Card />
            </div>
            <Clients />
        </>
    }
}
