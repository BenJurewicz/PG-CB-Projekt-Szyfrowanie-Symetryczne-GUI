use crate::components::FileDetails;
use crate::types::FileData;
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
pub fn FileCard(file: Signal<Option<FileData>>) -> Element {
    rsx! {
        div {
            if file.read().is_some() {
                FileDetails {file: file.unwrap()}
            }
        }

        button {
            class: "btn btn-secondary ml-4",
            onclick: move |_| async move {
                if let Some(f) = read_file().await {
                    file.set(Some(f));
                }
            },
            "Upload File"
        }

        button {
            class: "btn btn-accent ml-4",
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
