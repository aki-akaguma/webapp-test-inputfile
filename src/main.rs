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
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Test {}
    }
}

#[component]
fn Test() -> Element {
    let filename: Signal<String> = use_signal(String::new);
    //let mut string: Signal<String> = use_signal(String::new);
    let size: Signal<String> = use_signal(String::new);
    let content_type: Signal<String> = use_signal(String::new);
    let img_src: Signal<String> = use_signal(String::new);

    rsx! {
        div {
            input { id: "ti", r#type: "text" }
            br {}
            input {
                id: "fi",
                r#type: "file",
                accept: "image/*",
                multiple: false,
                onchange: move |evt| async move {
                    input_file_onchange_(evt, filename, size, content_type, img_src).await;
                },
            }
            div { "{filename}" }
            div { "{size}" }
            div { "{content_type}" }
            img {
                id: "ig1",
                src: "{img_src}",
                style: "max-width: 600px; margin-top: 20px;",
            }
        }
    }
}

async fn input_file_onchange_(
    evt: Event<FormData>,
    mut filename: Signal<String>,
    mut size: Signal<String>,
    mut content_type: Signal<String>,
    mut img_src: Signal<String>,
) {
    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
    use dioxus::html;
    //
    let v: &[html::FileData] = &evt.files();
    if !v.is_empty() {
        let file_data: &html::FileData = &v[0];
        filename.set(format!("name: {}", file_data.name()));
        size.set(format!(
            "size: {:.1} kb",
            (file_data.size() as f64) / 1024.0
        ));
        if let Some(s) = file_data.content_type() {
            content_type.set(format!("content type: {}", s));
        }
        dioxus_logger::tracing::debug!("PASS: 0: {}", file_data.path().display());
        if let Ok(bytes) = file_data.read_bytes().await {
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
