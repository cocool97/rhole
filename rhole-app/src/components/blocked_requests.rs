use crate::{components::InputList, RHOLE_CLIENT};
use chrono::{DateTime, NaiveDateTime, Utc};
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
            // Create a NaiveDateTime from the timestamp
            let naive =
                NaiveDateTime::from_timestamp_opt(request.timestamp.round() as i64, 0).unwrap();

            // Create a normal DateTime from the NaiveDateTime
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

            vec![
                request.request_id.to_string(),
                request.client_id.to_string(),
                request.request_address.to_string(),
                datetime.to_rfc3339(),
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
