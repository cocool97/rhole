use crate::{
    components::{InputList, UpdatedComponent},
    RHOLE_CLIENT,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use common::BlockedRequest;
use log::error;
use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

#[function_component]
pub fn BlockedRequests() -> Html {
    let blocked_requests = use_state(Vec::new);
    let blocked_requests_child = blocked_requests.clone();

    let callback: Callback<()> = Callback::from(move |_| {
        let blocked_requests = blocked_requests.clone();

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
    });

    html!(
        <UpdatedComponent
            tick_interval_ms={5 * 1000}
            update_callback={callback}
        >
            <InnerBlockedRequests
                blocked_requests={blocked_requests_child}
            />
        </UpdatedComponent>
    )
}

#[derive(Properties, PartialEq)]
pub struct InnerBlockedRequestsProps {
    pub blocked_requests: UseStateHandle<Vec<BlockedRequest>>,
}

#[function_component]
fn InnerBlockedRequests(props: &InnerBlockedRequestsProps) -> Html {
    let blocked_requests_list: Vec<Vec<String>> = props
        .blocked_requests
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
