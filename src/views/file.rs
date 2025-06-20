use crate::components::FileCard;
use crate::types::FileData;
use dioxus::prelude::*;

#[component]
pub fn File() -> Element {
    let decrypted_file: Signal<Option<FileData>> = use_signal(|| None);
    let encrypted_file: Signal<Option<FileData>> = use_signal(|| None);

    rsx! {
        h1 {
            class: "text-4xl font-bold text-center my-8",
            "File Upload and Download Example"
        }
        div {
            class: "flex-1 flex-row",
            div {
                class: "border border-base-300 rounded-lg p-4 m-4",
                p {
                    class: "text-center mb-4",
                    "Encrypted File"
                }
                FileCard {
                    file: encrypted_file
                }
            }

            div {
                class: "border border-base-300 rounded-lg p-4 m-4",
                p {
                    class: "text-center mb-4",
                    "Decrypted File"
                }
                FileCard {
                    file: decrypted_file
                }
            }
        }
    }
}
