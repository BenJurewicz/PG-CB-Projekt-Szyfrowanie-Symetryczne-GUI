use crate::EXCHANGE_SVG;
use dioxus::prelude::*;

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
            class: "text-center portrait:mx-2 mb-4",
            "Select an encryption mode and enter text to encrypt or decrypt."
        }
        div {
            class: "w-full max-w-7xl mx-auto px-4 sm:px-8 lg:px-16 landscape:mt-20",
            div {
                class: "flex flex-row items-center justify-start mb-4 portrait:justify-center",
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
                class: "flex justify-center mb-4",
                input {
                    type: "text",
                    id: "pwd",
                    name: "pwd",
                    class: "input input-bordered py-2 text-base w-full portrait:max-w-[530px]",
                    placeholder: "Password",
                }
            }

            div {
                class: "flex flex-row items-center justify-between w-full portrait:!flex-col portrait:!justify-center portrait:!items-center",
                textarea {
                    class: "textarea textarea-bordered portrait:w-full landscape:w-1/2 max-w-[530px] w-full",
                    placeholder: "Enter text to encrypt...",
                    rows: 10,
                }
                img {
                    src: "{EXCHANGE_SVG}",
                    alt: "Exchange Icon",
                    class: "mx-8 w-8 h-8 portrait:my-4 landscape:mx-8 portrait:rotate-90",
                }
                textarea {
                    class: "textarea textarea-bordered portrait:w-full landscape:w-1/2 max-w-[530px] w-full",
                    placeholder: "Enter text to decrypt here...",
                    rows: 10,
                }
            }
        }
    }
}
