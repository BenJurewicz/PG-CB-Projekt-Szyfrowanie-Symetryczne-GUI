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
            class: "text-center mb-4 portrait:mx-2",
            "Encrypt and decrypt text using AES encryption in different modes."
        }
        div {
            class: "flex-col max-w-7xl mx-auto px-4 sm:px-8 lg:px-16 landscape:mt-20",
            fieldset {
                class: "border-2 border-base-300 rounded-2xl px-4 py-2 mb-6 landscape:w-1/2 max-w-[530px] portrait:mx-auto",
                legend {
                    class: "font-semibold text-primary-content px-2",
                    "Select encryption mode"
                }
                div {
                    class: "flex flex-row items-center justify-start gap-4 portrait:justify-center max-[380px]:flex-col max-[376px]:gap-4 mb-2",
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
                        class: "label cursor-pointer mx-4 text-primary-content",
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
                        class: "label cursor-pointer mx-4 text-primary-content",
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
                        class: "label cursor-pointer mx-4 text-primary-content",
                        "CTR Mode"
                    }
                }
            }

            div {
                class: "flex flex-row items-center justify-between w-full landscape:mt-15 portrait:!flex-col portrait:!justify-center portrait:!items-center landscape:mb-10 portrait:mb-5",
                // LEFT COLUMN
                div {
                    class: "flex flex-col w-full landscape:w-1/2 max-w-[530px]",
                    label {
                        for: "pwd",
                        class: "mb-2 text-primary-content",
                        "Password:"
                    }
                    input {
                        type: "text",
                        id: "pwd",
                        name: "pwd",
                        class: "input input-bordered py-2 text-base w-full mb-4",
                        placeholder: "Enter your password",
                    }
                    label {
                        for: "encrypt-text",
                        class: "mb-1 text-primary-content",
                        "Text to encrypt:"
                    }
                    textarea {
                        id: "encrypt-text",
                        class: "textarea textarea-bordered w-full",
                        placeholder: "Enter text to encrypt...",
                        rows: 10,
                    }
                }
                // IMAGE
                img {
                    src: "{EXCHANGE_SVG}",
                    alt: "Exchange Icon",
                    class: "w-8 h-8 landscape:mx-8 landscape:mt-25 portrait:my-2 portrait:mt-8 portrait:rotate-90",
                }
                // RIGHT COLUMN
                div {
                    class: "flex flex-col w-full landscape:w-1/2 max-w-[530px]",
                    // Invisible placeholders for alignment
                    label {
                        class: "mb-2 invisible portrait:hidden",
                        "Password:"
                    }
                    input {
                        class: "input input-bordered py-2 text-base w-full mb-4 invisible portrait:hidden",
                    }
                    label {
                        for: "decrypt-text",
                        class: "mb-1 text-white",
                        "Text to decrypt:"
                    }
                    textarea {
                        id: "decrypt-text",
                        class: "textarea textarea-bordered w-full",
                        placeholder: "Enter text to decrypt here...",
                        rows: 10,
                    }
                }
            }
        }
    }
}
