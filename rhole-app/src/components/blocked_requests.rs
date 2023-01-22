use crate::RHOLE_CLIENT;
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

    let blocked_requests_list: Vec<Html> = blocked_requests
        .iter()
        .map(|request| {
            html! {
                <div>
                    <h1>{"Request ID: "}{request.request_id}</h1>
                    <p>{"Client_id: "} {request.client_id}</p>
                    <p>{"Request address: "}{request.request_address.clone()}</p>
                    <p>{"Timestamp: "}{request.timestamp}</p>
                </div>
            }
        })
        .collect();

    html! {
        <div>
            {blocked_requests_list}
        </div>
    }
}
