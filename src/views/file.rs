use crate::components::{CryptoOptions, ExchangeSVG, FileCard};
use crate::types::FileData;
use crate::utils::CipherMode;
use dioxus::prelude::*;

#[component]
pub fn File() -> Element {
    let decrypted_file: Signal<Option<FileData>> = use_signal(|| None);
    let encrypted_file: Signal<Option<FileData>> = use_signal(|| None);

    let mode = use_signal(|| CipherMode::Ctr);
    let password = use_signal(String::new);

    rsx! {
        h1 {
            class: "text-4xl font-bold text-center my-8",
            "File Upload and Download Example"
        }

        p {
            class: "text-center mb-4 portrait:mx-2",
            "Encrypt and decrypt files using AES encryption in different modes."
        }

        CryptoOptions{
            mode: mode,
            password: password,
        }

        // TODO
        // button {
        //     class: "btn btn-primary my-4",
        //     "Decrypt File"
        // }

        div {
            class: "flex lg:flex-row items-center flex-col justify-center gap-4 landscape:mt-25",

            FileCard {
                title: "Encrypted File",
                file: encrypted_file
            }

            ExchangeSVG {
                class: "max-lg:rotate-90"
            }

            FileCard {
                title: "Decrypted File",
                file: decrypted_file
            }
        }
    }
}
