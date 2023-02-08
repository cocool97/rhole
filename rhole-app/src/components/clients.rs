use crate::{components::InputList, RHOLE_CLIENT};
use chrono::{DateTime, NaiveDateTime, Utc};
use common::Client;
use log::error;
use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

use crate::components::UpdatedComponent;

#[function_component]
pub fn Clients() -> Html {
    let all_clients: UseStateHandle<Vec<Client>> = use_state(Vec::new);
    let all_clients_child = all_clients.clone();

    let callback: Callback<()> = Callback::from(move |_| {
        let all_clients = all_clients.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let response = RHOLE_CLIENT.clients().await;
            match response {
                Ok(clients) => {
                    all_clients.set(clients);
                }
                Err(e) => {
                    error!("Error encountered: {e}");
                    all_clients.set(vec![]);
                }
            }
        });
    });

    html!(
        <UpdatedComponent
            tick_interval_ms={5 * 1000}
            update_callback={callback}
        >
            <InnerClients
                all_clients={all_clients_child}
            />
        </UpdatedComponent>

    )
}

#[derive(Properties, PartialEq)]
pub struct InnerClientsProps {
    pub all_clients: UseStateHandle<Vec<Client>>,
}

#[function_component]
pub fn InnerClients(props: &InnerClientsProps) -> Html {
    let clients_list: Vec<Vec<String>> = props
        .all_clients
        .iter()
        .map(|user| {
            let naive =
                NaiveDateTime::from_timestamp_opt(user.last_seen.round() as i64, 0).unwrap();
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
