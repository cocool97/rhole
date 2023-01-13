use yew::{function_component, html, Html};

#[function_component]
pub fn Title() -> Html {
    html! {
        <div>
            <h1>{"Rhole: Local DNS adblocker"}</h1>
        </div>
    }
}
