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
            class: "flex flex-row portrait:flex-col max-[1100px]:flex-col justify-center gap-4 landscape:mx-10 landscape:mt-25",
            div {
                class: "flex-1 min-w-0 border-3 border-base-300 rounded-lg p-4 m-4",
                p {
                    class: "text-center mb-4",
                    "Encrypt File"
                }
                div {
                class: "flex portrait:flex-col max-[1600px]:flex-col landscape:gap-15 portrait:gap-4 justify-center items-center mb-4 landscape:my-10",
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
                div {
                class: "flex portrait:flex-col max-[1600px]:flex-col landscape:gap-15 portrait:gap-4 justify-center items-center mb-4 landscape:my-10",
                fieldset {
                    class: "fieldset landscape:grid-flow-col bg-base-200 border-base-300 rounded-box w-xs border portrait:mx-auto",
                    legend {
                        class: "fieldset-legend",
                        "Select decryption mode"
                    }
                        label {
                            for: "ecb",
                            class: "label cursor-pointer mx-4 text-primary-content text-base",
                            input {
                                type: "radio",
                                name: "decryption_mode",
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
                                name: "decryption_mode",
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
                                name: "decryption_mode",
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
                FileCard {
                    file: decrypted_file
                }
            }
        }
    }
}
