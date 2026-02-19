use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut filename: Signal<String> = use_signal(String::new);
    //let mut string: Signal<String> = use_signal(String::new);
    let mut size: Signal<String> = use_signal(String::new);
    let mut content_type: Signal<String> = use_signal(String::new);
    let mut img_src: Signal<String> = use_signal(String::new);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            input {
                id: "ti",
                r#type: "text",
            }
            br {}
            input {
                id: "fi",
                r#type: "file",
                accept: "image/*",
                multiple: false,
                onchange: move |evt: Event<FormData>| async move {
                    use dioxus::html;
                    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
                    //
                    let v: &[html::FileData] = &evt.files();
                    if !v.is_empty() {
                        let f: &html::FileData = &v[0];
                        filename.set(f.name());
                        size.set(format!("size: {:.1} kb", (f.size() as f64)/1024.0));
                        if let Some(s) = f.content_type() {
                            content_type.set(format!("content type: {}",s));
                        }
                        dioxus_logger::tracing::debug!("PASS: 0: {}", f.path().display());
                        if let Ok(bytes) = f.read_bytes().await {
                            let base64 = STANDARD_NO_PAD.encode(&bytes);
                            //
                            // data uri schema
                            // <img src="data:[<mediatype>][;base64],<data>" alt="Base64 Image">
                            //let data_url = format!("data:image/png;base64,{}", base64);
                            let data_url = format!("data:;base64,{}", base64);
                            img_src.set(data_url);
                        }
                    }
                }
            }
            div {
                "{filename}"
            }
            div {
                "{size}"
            }
            div {
                "{content_type}"
            }
            img {
                id: "ig1",
                src: "{img_src}",
                style: "max-width: 300px; margin-top: 20px;"
            }
        }
    }
}
