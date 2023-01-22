use crate::RHOLE_CLIENT;
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

    let clients_list: Vec<Html> = all_clients
        .iter()
        .map(|user| {
            html! {
                <div>
                    <h1>{"Client: "}{user.client_id}</h1>
                    <p>{"IP address: "} {user.address.clone()}</p>
                    <p>{"Last seen: "}{user.last_seen}</p>
                </div>
            }
        })
        .collect();

    html! {
        <div>
            {clients_list}
        </div>
    }
}
