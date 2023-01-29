use crate::{components::InputList, RHOLE_CLIENT};
use common::BlockedRequest;
use log::error;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, UseStateHandle};

#[function_component]
pub fn BlockedRequests() -> Html {
    let blocked_requests: UseStateHandle<Vec<BlockedRequest>> = use_state(Vec::new);

    {
        let blocked_requests = blocked_requests.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = RHOLE_CLIENT.blocked_requests().await;
                    match response {
                        Ok(clients) => blocked_requests.set(clients),
                        Err(e) => {
                            error!("Error encountered: {e}");
                            blocked_requests.set(vec![])
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let blocked_requests_list: Vec<Vec<String>> = blocked_requests
        .iter()
        .map(|request| {
            vec![
                request.request_id.to_string(),
                request.client_id.to_string(),
                request.request_address.to_string(),
                request.timestamp.to_string(),
            ]
        })
        .collect();

    html! {
        <InputList
            header={vec!["Request ID", "Client ID", "Request address", "Timestamp"]}
            input={blocked_requests_list}
        />
    }
}
