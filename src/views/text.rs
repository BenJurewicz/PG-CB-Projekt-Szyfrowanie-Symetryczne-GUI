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
            div {
                class: "flex portrait:flex-col landscape:gap-15 portrait:gap-4 justify-center portrait:items-center mb-4",
                fieldset {
                    class: "fieldset landscape:grid-flow-col bg-base-200 border-base-300 rounded-box w-xs border portrait:mx-auto",
                    legend {
                        class: "fieldset-legend",
                        "Select encryption mode"
                    }
                        label {
                            for: "ecb",
                            class: "label cursor-pointer mx-4 text-primary-content text-base",
                            input {
                                type: "radio",
                                name: "encryption_mode",
                                id: "ecb",
                                class: "radio radio-primary m-1",
                                value: "ECB",
                                checked: true
                            }
                            "ECB Mode"
                        }
                        label {
                            for: "cbc",
                            class: "label cursor-pointer mx-4 text-primary-content text-base",
                            input {
                                type: "radio",
                                name: "encryption_mode",
                                id: "cbc",
                                class: "radio radio-primary m-1",
                                value: "CBC"
                            }
                            "CBC Mode"
                        }
                        label {
                            for: "ctr",
                            class: "label cursor-pointer mx-4 text-primary-content text-base",
                            input {
                                type: "radio",
                                name: "encryption_mode",
                                id: "ctr",
                                class: "radio radio-primary m-1",
                                value: "CTR"
                            }
                            "CTR Mode"
                        }
                }
                fieldset {
                    class: "fieldset max-w-1/2",
                    legend {
                        class: "fieldset-legend py-1.5",
                        "Password"
                    }
                    input {
                        type: "text",
                        id: "pwd",
                        name: "pwd",
                        class: "input input-bordered text-base w-full",
                        placeholder: "Enter your password",
                    }
                }
            }

            div {
                class: "flex flex-row items-center justify-between w-full landscape:mt-15 portrait:!flex-col portrait:!justify-center portrait:!items-center landscape:mb-10 portrait:mb-5",
                // LEFT COLUMN
                div {
                    class: "flex flex-col w-full landscape:w-1/2 max-w-[530px]",
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
                    class: "w-8 h-8 landscape:mx-8 landscape:mt-10 portrait:my-2 portrait:mt-8 portrait:rotate-90",
                }
                // RIGHT COLUMN
                div {
                    class: "flex flex-col w-full landscape:w-1/2 max-w-[530px]",
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
