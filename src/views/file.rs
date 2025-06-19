use crate::components::FileCard;
use crate::types::FileData;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;

#[component]
pub fn File() -> Element {
    // TODO Instead of vec for files,
    // TODO Have two variables: decrypted_file and encrypted_file
    // let mut files_uploaded = use_signal(|| Vec::new() as Vec<FileData>);

    let decrypted_file: Signal<Option<FileData>> = use_signal(|| None);
    let encrypted_file: Signal<Option<FileData>> = use_signal(|| None);

    rsx! {
        h1 {
            class: "text-4xl font-bold text-center my-8",
            "File Encryption and Decryption"
        }
        div {
            class: "flex landscape:flex-row portrait:flex-col justify-center gap-4 landscape:mx-10 landscape:mt-25",
            div {
                class: "flex-1 min-w-0 border-3 border-base-300 rounded-lg p-4 m-4",
                p {
                    class: "text-center mb-4",
                    "Encrypt File"
                }
                FileCard {
                    file: encrypted_file
                }
            }

            div {
                class: "flex-1 min-w-0 border-3 border-base-300 rounded-lg p-4 m-4",
                p {
                    class: "text-center mb-4",
                    "Decrypt File"
                }
                FileCard {
                    file: decrypted_file
                }
            }
        }
    }
}
