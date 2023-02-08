use yew::{function_component, html, Html};

use crate::components::InputList;

#[function_component]
pub fn BlockedDomains() -> Html {
    html! {
        <InputList
            header={vec!["Domain ID", "Blocked domain", "Date added", "Requests blocked", "Unblock"]}
            input={vec![]}
        />
    }
}
