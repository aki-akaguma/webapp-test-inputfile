use dioxus::prelude::*;

#[cfg(feature = "desktop")]
use anyhow::Result;
#[cfg(feature = "desktop")]
use serde::{Deserialize, Serialize};

const APP_PNG: Asset = asset!("/assets/app.png");
const TEST1_JS: Asset = asset!("/assets/js/test1.js", AssetOptions::js().with_minify(true));

#[cfg(feature = "desktop")]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct AnchorInfo {
    pub download: Option<String>,
    pub href: Option<String>,
}
#[cfg(feature = "desktop")]
impl AnchorInfo {
    pub fn from_json_str(s: &str) -> Result<Self> {
        let r = serde_json::from_str(s)?;
        Ok(r)
    }
}

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
            a {
                id: "lnk1",
                onclick: move |evt| async move {
                    download_file("lnk1").await;
                    evt.stop_propagation();
                },
                download: "app.png",
                href: APP_PNG,
                "Download Link: asset file"
            }
            br {}
            // download link: data
            a {
                id: "lnk2",
                onclick: move |evt| async move {
                    download_file("lnk2").await;
                    evt.stop_propagation();
                },
                download: "{dl_data_fnm}",
                href: "{dl_data_src}",
                "{dl_data_msg}"
            }
            br {}
            // download link: blob
            a {
                id: "lnk3",
                /*
                onclick: move |evt| async move {
                    download_file("lnk3").await;
                    evt.stop_propagation();
                },
                */
                target: "_blank",
                onclick: move |evt| {
                    evt.stop_propagation();
                    spawn(async move{
                        download_file("lnk3").await;
                    });
                },
                "{dl_blob_msg}"
            }
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
                    "setDataUrlToBlobLink('{}','{}','{}','{}');",
                    data_url,
                    mime_type,
                    file_data.name(),
                    "lnk3"
                );
                let _ = document::eval(&js).await;
                dl_blob_msg.set("Download Link: Blob".to_string());
            }
        }
    }
}

#[cfg(not(feature = "desktop"))]
async fn download_file(_id: &str) {}

#[cfg(feature = "desktop")]
async fn download_file(id: &str) {
    //dioxus_logger::tracing::debug!("data: {:?}", evt.data());
    let js = format!(r#"{{return getAnchorsDownloadHref('{}');}}"#, id);
    let v = document::eval(&js).await.unwrap();
    let s = v.to_string();
    let anchorinfo = AnchorInfo::from_json_str(&s).unwrap();
    let filename = anchorinfo.download.unwrap();
    dioxus_logger::tracing::debug!("filename: {filename}");
    if let Some(path) = rfd::FileDialog::new().set_file_name(filename).save_file() {
        let content = anchorinfo.href.unwrap();
        let is_data = content.starts_with("data:");
        let is_blob = content.starts_with("blob:");
        if is_data || is_blob {
            let data_url = if is_blob {
                let js = format!(r#"{{parseBlobData_dxsend('{}');}}"#, content);
                let mut eval = document::eval(&js);
                let data_url = eval.recv::<String>().await.unwrap();
                data_url
            } else {
                content
            };
            save_data_uri0(&data_url, &path).unwrap();
        }
    }
}

#[cfg(feature = "desktop")]
fn save_data_uri0(url: &str, path: &std::path::PathBuf) -> Result<()> {
    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
    // data uri schema:
    //   data:[<mediatype>][;base64],<data>
    let base64_s = url.split_once(',').unwrap().1;
    let v = STANDARD_NO_PAD.decode(base64_s)?;
    if let Ok(mut file) = std::fs::File::create(path) {
        use std::io::Write;
        let _ = file.write_all(&v);
    }
    Ok(())
}
