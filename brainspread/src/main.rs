#![allow(non_snake_case)]

use color_eyre::eyre::Result;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_desktop::Config;

use brainspread::config::Config as AppConfig;
use brainspread::get_config;
use brainspread::services::openai::generate_tags;

struct AppProps {
    config: AppConfig,
}

fn main() -> Result<()> {
    let config = get_config()?;

    // launch the dioxus app in a webview
    dioxus_desktop::launch_with_props(App, AppProps { config }, Config::new());

    Ok(())
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope<AppProps>) -> Element {
    let links = use_state(&cx, || "".to_string());
    let tags = use_state(&cx, || "".to_string());

    let loading_class = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        head {
            link {
                rel: "stylesheet",
                href: "https://unpkg.com/bulma@0.9.0/css/bulma.min.css",
            }
        }
        div { class: "container", style: "margin: 15px;",
        div { class: "columns",
            div { class: "column",
                textarea { class: "textarea is-primary mt-4", rows: "10",
                value:"{links}",
                    placeholder: "comma separated list of links",
                    oninput: move |evt| {
                        links.set(evt.value.clone());
                    },
                }
                textarea { class: "textarea is-primary mt-4", rows: "10",
                value:"{tags}",
                    placeholder: "",
                }
            }
        }

        button { class: "button is-primary {loading_class}",
        onclick: move |_| cx.spawn({
            let loading_class = loading_class.clone();
            loading_class.set("is-loading".to_string());
            let links = links.clone();
            let tags = tags.clone();
            // TODO - validation
            async move {
                if let Ok(generated_tags) = generate_tags(&links).await {
                    let generated_tags = generated_tags.join(", ");
                    tags.set(generated_tags.to_string());
                }
                loading_class.set("".to_string());
            }
        }),
            "Generate labels"
        }
        br {}
    }})
}
