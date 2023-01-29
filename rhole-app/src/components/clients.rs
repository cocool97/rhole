use crate::{components::InputList, RHOLE_CLIENT};
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
                            all_clients.set(vec![])
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
            vec![
                user.client_id.to_string(),
                user.address.to_string(),
                user.last_seen.to_string(),
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
