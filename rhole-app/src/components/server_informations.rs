use common::Infos;
use log::error;
use yew::{function_component, html, use_effect_with_deps, use_state_eq, Html};

use crate::RHOLE_CLIENT;

#[function_component]
pub fn ServerInformations() -> Html {
    let infos = use_state_eq(Infos::default);

    {
        let infos = infos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = RHOLE_CLIENT.infos().await;
                    match response {
                        Ok(clients) => infos.set(clients),
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

    html! {
        <>
            <h1>{"Server informations"}</h1>
            <ul>
                <li>
                    <h3>{"Uptime: "}</h3><p>{infos.uptime.to_owned()}</p>
                </li>
                <li>
                    <h3>{"Version: "}</h3>{infos.build_version.to_owned()}
                </li>
                <li>
                    <h3>{"Build timestamp: "}</h3>{infos.build_timestamp.to_owned()}
                </li>
                <li>
                    <h3>{"GIT commit SHA: "}</h3>{infos.git_commit_sha.to_owned()}
                </li>
                <li>
                    <h3>{"GIT commit date: "}</h3>{infos.git_commit_date.to_owned()}
                </li>
                <li>
                    <h3>{"GIT commit branch: "}</h3>{infos.git_commit_branch.to_owned()}
                </li>
                <li>
                    <h3>{"rustc version: "}</h3>{infos.rustc_version.to_owned()}
                </li>
                <li>
                    <h3>{"rustc channel: "}</h3>{infos.rustc_channel.to_owned()}
                </li>
                <li>
                    <h3>{"rustc host triple: "}</h3>{infos.rustc_host_triple.to_owned()}
                </li>
                <li>
                    <h3>{"rustc commit SHA: "}</h3>{infos.rustc_commit_sha.to_owned()}
                </li>
                <li>
                    <h3>{"Build OS version: "}</h3>{infos.build_os_version.to_owned()}
                </li>
                <li>
                    <h3>{"Build CPU vendor: "}</h3>{infos.build_cpu_vendor.to_owned()}
                </li>
                <li>
                    <h3>{"cargo target: "}</h3>{infos.cargo_target.to_owned()}
                </li>
                <li>
                    <h3>{"cargo profile: "}</h3>{infos.cargo_profile.to_owned()}
                </li>
                <li>
                    <h3>{"cargo features: "}</h3>{infos.cargo_features.to_owned()}
                </li>
          </ul>
        </>
    }
}
