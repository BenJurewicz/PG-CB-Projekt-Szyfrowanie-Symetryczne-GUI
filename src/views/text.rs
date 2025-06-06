use dioxus::prelude::*;
use crate::EXCHANGE_SVG;

#[component]
pub fn Text() -> Element {
    rsx! {
        h1 {
            class: "text-4xl font-bold text-center my-8",
            "Text Encryption"
        }

        p {
            class: "text-center mb-4",
            "Encrypt and decrypt text using AES encryption in different modes."
        }
        p {
            class: "text-center mb-4",
            "Select an encryption mode and enter text to encrypt."
        }
        div {
            class: "flex items-left mx-auto ml-40 mb-8 mt-20 sm:px-8 lg:px-16",
            input {
                type: "radio",
                name: "encryption_mode",
                id: "ecb",
                class: "radio radio-primary",
                value: "ECB",
                checked: true
            }
            label {
                for: "ecb",
                class: "label cursor-pointer mx-4 text-white",
                "ECB Mode"
            }
            input {
                type: "radio",
                name: "encryption_mode",
                id: "cbc",
                class: "radio radio-primary",
                value: "CBC"
            }
            label {
                for: "cbc",
                class: "label cursor-pointer mx-4 text-white",
                "CBC Mode"
            }
            input {
                type: "radio",
                name: "encryption_mode",
                id: "ctr",
                class: "radio radio-primary",
                value: "CTR"
            }
            label {
                for: "ctr",
                class: "label cursor-pointer mx-4 text-white",
                "CTR Mode"
            }
        }

        div {
            class: "mx-auto ml-55 my-8",
            input {
                type: "password",
                id: "pwd",
                name: "pwd",
                class: "input input-bordered", // Optional Tailwind CSS class for styling
                placeholder: "Password", // Label text inside the textbox
            }
        }

        div {
            class: "flex items-center justify-between w-full mx-auto px-4 sm:px-8 lg:px-16",
            textarea {
                class: "textarea textarea-bordered w-1/2 ml-40",
                placeholder: "Enter text to encrypt...",
                rows: 10,
            }
            img {
                src: "{EXCHANGE_SVG}",
                alt : "Exchange Icon",
                class: "mx-4 w-8 h-8",
            }
            textarea {
                class: "textarea textarea-bordered w-1/2 mr-40",
                placeholder: "Enter text to decrypt here...",
                rows: 10,
            }
        }
    }
}
