use crate::{components::InputList, RHOLE_CLIENT};
use chrono::{DateTime, NaiveDateTime, Utc};
use common::Client;
use log::error;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, UseStateHandle};

#[function_component]
pub fn Clients() -> Html {
    let all_clients: UseStateHandle<Vec<Client>> = use_state(Vec::new);

    {
        let all_clients = all_clients.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = RHOLE_CLIENT.clients().await;
                    match response {
                        Ok(clients) => all_clients.set(clients),
                        Err(e) => {
                            error!("Error encountered: {e}");
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let clients_list: Vec<Vec<String>> = all_clients
        .iter()
        .map(|user| {
            // Create a NaiveDateTime from the timestamp
            let naive =
                NaiveDateTime::from_timestamp_opt(user.last_seen.round() as i64, 0).unwrap();

            // Create a normal DateTime from the NaiveDateTime
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

            vec![
                user.client_id.to_string(),
                user.address.to_string(),
                datetime.to_rfc3339(),
            ]
        })
        .collect();

    html! {
        <div>
            <InputList
                header={vec!["Client ID", "IP Address", "Last seen"]}
                input={clients_list}
            />
        </div>
    }
}
