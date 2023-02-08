use crate::{components::InputList, RHOLE_CLIENT};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::error;
use yew::{function_component, html, use_effect_with_deps, use_state_eq, Html};

#[function_component]
pub fn BlockedDomains() -> Html {
    let blocked_domains = use_state_eq(Vec::new);

    {
        let blocked_domains = blocked_domains.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = RHOLE_CLIENT.blocked_domains().await;
                    match response {
                        Ok(clients) => blocked_domains.set(clients),
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

    let blocked_domains_list: Vec<Vec<String>> = blocked_domains
        .iter()
        .map(|request| {
            // Create a NaiveDateTime from the timestamp
            let naive =
                NaiveDateTime::from_timestamp_opt(request.insert_timestamp.round() as i64, 0)
                    .unwrap();

            // Create a normal DateTime from the NaiveDateTime
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

            vec![
                request.domain_id.to_string(),
                request.domain_address.to_string(),
                datetime.to_rfc3339(),
                request.blocked_count.to_string(),
                request.whitelisted.to_string(),
            ]
        })
        .collect();

    html! {
        <InputList
                header={vec!["Domain ID", "Blocked domain", "Date added", "Total requests blocked", "Unblocked"]}
                input={blocked_domains_list}
            />
    }
}
