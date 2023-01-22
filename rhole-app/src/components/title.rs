use yew::{function_component, html, Html};

#[function_component]
pub fn Title() -> Html {
    html! {
        <div class="flex" style="border-radius: 25px; border: 2px solid black; padding: 15px; margin-bottom: 50px;">
            <div class="flex" style="flex-direction: column">
                <h1 style="margin: 0">{"Rhole"}</h1>
                <h3 style="margin: 0">{"Local DNS blocker - privacy & advertisment"}</h3>
            </div>
        </div>
    }
}
