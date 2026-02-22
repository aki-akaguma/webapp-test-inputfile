use dioxus::prelude::*;

const APP_PNG: Asset = asset!("/assets/app.png");
const TEST1_JS: Asset = asset!("/assets/js/test1.js", AssetOptions::js().with_minify(false));

#[component]
pub fn Test1() -> Element {
    let filename: Signal<String> = use_signal(String::new);
    let size: Signal<String> = use_signal(String::new);
    let content_type: Signal<String> = use_signal(String::new);
    let mime_type: Signal<String> = use_signal(String::new);
    let img_src: Signal<String> = use_signal(String::new);
    let dl_data_fnm: Signal<String> = use_signal(String::new);
    let dl_data_src: Signal<String> = use_signal(String::new);
    let dl_data_msg: Signal<String> = use_signal(String::new);
    let dl_blob_msg: Signal<String> = use_signal(String::new);

    rsx! {
        document::Script { src: TEST1_JS }
        div {
            input {
                id: "fi",
                r#type: "file",
                accept: "image/*",
                multiple: false,
                onchange: move |evt| async move {
                    input_file_onchange_(
                            evt,
                            filename,
                            size,
                            content_type,
                            mime_type,
                            img_src,
                            dl_data_fnm,
                            dl_data_src,
                            dl_data_msg,
                            dl_blob_msg,
                        )
                        .await;
                },
            }
            div { "{filename}" }
            div { "{size}" }
            div { "{content_type}" }
            div { "{mime_type}" }
            img {
                id: "ig1",
                src: "{img_src}",
                style: "max-width: 360px; margin-top: 20px; margin-left: 0;  margin-right: 0",
            }
            br {}
            // download link: asset file
            a { id: "lnk1", download: "app.png", href: APP_PNG, "Download Link: asset file" }
            br {}
            // download link: data
            a {
                id: "lnk2",
                download: "{dl_data_fnm}",
                href: "{dl_data_src}",
                "{dl_data_msg}"
            }
            br {}
            // download link: blob
            a { id: "lnk3", "{dl_blob_msg}" }
        }
    }
}

async fn input_file_onchange_(
    evt: Event<FormData>,
    mut filename: Signal<String>,
    mut size: Signal<String>,
    mut content_type: Signal<String>,
    mut mime_type_sig: Signal<String>,
    mut img_src: Signal<String>,
    mut dl_data_fnm: Signal<String>,
    mut dl_data_src: Signal<String>,
    mut dl_data_msg: Signal<String>,
    mut dl_blob_msg: Signal<String>,
) {
    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
    use dioxus::html;
    //
    let v: &[html::FileData] = &evt.files();
    if !v.is_empty() {
        let file_data: &html::FileData = &v[0];
        let fnm = file_data.name();
        //
        filename.set(format!("name: {}", fnm));
        size.set(format!(
            "size: {:.1} kb",
            (file_data.size() as f64) / 1024.0
        ));
        let mime_type = {
            let mime_type = if let Some(s) = file_data.content_type() {
                content_type.set(format!("content type: {}", s));
                s
            } else {
                "".to_string()
            };
            if mime_type.starts_with("image/") {
                mime_type
            } else {
                if let Some(a) = fnm.rsplit_once('.') {
                    match a.1.to_uppercase().as_str() {
                        "JPEG" | "JPG" => "image/jpeg",
                        "PNG" => "image/png",
                        "GIF" => "image/gif",
                        "WEBP" => "image/webp",
                        _ => "",
                    }
                } else {
                    ""
                }
                .to_string()
            }
        };
        mime_type_sig.set(format!("mime type: {}", mime_type.clone()));
        dioxus_logger::tracing::debug!("PASS: 0: {}", file_data.path().display());
        if let Ok(bytes) = file_data.read_bytes().await {
            let base64 = STANDARD_NO_PAD.encode(&bytes);
            //
            // data uri schema
            // <img src="data:[<mediatype>][;base64],<data>" alt="Base64 Image">
            //let data_url = format!("data:image/png;base64,{}", base64);
            let data_url = format!("data:{};base64,{}", mime_type, base64);
            img_src.set(data_url.clone());
            dl_data_fnm.set(fnm.clone());
            dl_data_src.set(data_url.clone());
            dl_data_msg.set("Download Link: Data".to_string());
            //
            {
                let js = format!(
                    "setDataToBlobLink('{}','{}', '{}', '{}');",
                    data_url,
                    file_data.name(),
                    mime_type,
                    "lnk3"
                );
                let _ = document::eval(&js).await;
                dl_blob_msg.set("Download Link: Blob".to_string());
            }
        }
    }
}
