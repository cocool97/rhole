use common::ServerConfig;
use log::error;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, UseStateHandle};

use crate::RHOLE_CLIENT;

fn display_config(config: &Option<ServerConfig>) -> Html {
    match config {
        Some(c) => html!({ format!("{c:?}") }),
        None => html!(<></>),
    }
}

#[function_component]
pub fn Config() -> Html {
    let server_config: UseStateHandle<Option<ServerConfig>> = use_state(|| None);

    {
        let server_config = server_config.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = RHOLE_CLIENT.config().await;
                    match response {
                        Ok(config) => server_config.set(Some(config)),
                        Err(e) => {
                            error!("Error encountered: {e}");
                            server_config.set(None)
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            {display_config(&server_config)}
        </>
    }
}
