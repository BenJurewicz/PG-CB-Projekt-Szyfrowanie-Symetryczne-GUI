use dioxus::html::FileEngine;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use std::sync::Arc;

struct UploadedFile {
    name: String,
    contents: Vec<u8>,
}

impl UploadedFile {
    fn bin_as_hex_string(&self) -> String {
        self.contents
            .iter()
            .map(|&byte| format!("0x{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn content_as_string(&self) -> String {
        String::from_utf8_lossy(&self.contents).to_string()
    }
}

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);

    rsx! {
                h1 {
                    class: "text-4xl font-bold text-center my-8",
                }

                button {
                    class: "btn btn-accent ml-4",
                    onclick: move |_| async move {
                        let file = AsyncFileDialog::new()
                            .set_file_name("example.txt")
                            .save_file()
                            .await;
                        if file.is_none() {
                            return;
                        }
                        let file = file.unwrap();
                        file.write(b"Hello, this is a test file for download!").await.expect("TODO: panic message");
                    },
                    "Download File"
                }

                button {
                    class: "btn btn-secondary ml-4",
                    onclick: move |_| async move {
                        let file = AsyncFileDialog::new().pick_file().await;
                        if file.is_none() {
                            return;
                        }
                        let file = file.unwrap();
                        let name = file.file_name();
                        let data = file.read().await;
                        files_uploaded.write().clear();
                        files_uploaded.write().push(UploadedFile {
                            name: name.to_string(),
                            contents: data,
                        });
                    },
                    "Upload File"
                }
        for file in files_uploaded.read().iter().rev() {
            div {
                class: "card card-border bg-base-300 m-4 max-w-full md:max-w-2xl",
                div {
                    class: "card-body p-4",
                    span { class: "card-title text-primary-content flex justify-between items-center",
                        "{file.name}"
                        span { class: "text-xs text-gray-500", "{file.contents.len()} bytes" }
                    }

                    div {
                        class: "mt-2",
                        label { class: "font-semibold text-sm", "Text content:" }
                        div {
                            class: "bg-base-200 rounded-md p-2 mt-1 max-h-60 overflow-y-auto",
                            pre { class: "text-wrap text-secondary whitespace-pre-wrap", "{file.content_as_string()}" }
                        }
                    }

                    details {
                        class: "mt-4",
                        summary { class: "cursor-pointer font-semibold text-sm", "Hex view" }
                        div {
                            class: "bg-base-200 rounded-md p-2 mt-1 max-h-60 overflow-y-auto",
                            pre { class: "text-wrap text-accent break-all", "{file.bin_as_hex_string()}" }
                        }
                    }
                }
            }
        }
    }
}
