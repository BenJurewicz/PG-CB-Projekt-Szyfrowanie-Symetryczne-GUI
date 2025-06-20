use crate::components::FileDetails;
use crate::types::{DefaultView, FileData};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;

async fn read_file() -> Option<FileData> {
    let file = AsyncFileDialog::new().pick_file().await?;
    let name = file.file_name();
    let data = file.read().await;

    Some(FileData::new(name.to_string(), data))
}

async fn save_file(file: &FileData) {
    let file_handle = AsyncFileDialog::new()
        .set_file_name(file.name.clone())
        .save_file()
        .await;
    if file_handle.is_none() {
        return;
    }
    let file_handle = file_handle.unwrap();

    if let Err(e) = file_handle.write(&file.contents).await {
        tracing::error!("Failed to write file: {}", e);
    }
}

#[component]
pub fn FileCard(
    title: String,
    file: Signal<Option<FileData>>,
    oninput: Option<Callback>,
    default_view: DefaultView,
) -> Element {
    rsx! {
        div {
            class: "border border-base-300 rounded-lg p-4 m-4 max-w-xl",

            p {
                class: "text-center mb-4",
                "{title}"
            }

            div {
                if file.read().is_some() {
                    FileDetails {
                        file: file.unwrap(),
                        default_view: default_view,
                    }
                }
            }

            div {
                class: "flex justify-center items-center",

                button {
                    class: "btn btn-secondary my-4",
                    onclick: move |_| async move {
                        if let Some(c) = oninput {
                            c.call(());
                        }
                        if let Some(f) = read_file().await {
                            file.set(Some(f));
                        }
                    },
                    "Upload File"
                }

                button {
                    class: "btn btn-accent m-4",
                    onclick: move |_| async move {
                        let file_cpy = file();
                        if let Some(f) = file_cpy {
                            save_file(&f).await;
                        }
                    },
                    disabled: file.read().is_none(),
                    "Download File"
                }
            }
            }
    }
}
