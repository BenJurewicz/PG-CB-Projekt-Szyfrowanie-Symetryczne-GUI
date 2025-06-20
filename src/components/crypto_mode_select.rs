use crate::utils::CipherMode;
use dioxus::prelude::*;

#[component]
pub fn CryptoModeSelect(mode: Signal<CipherMode>, class: Option<String>) -> Element {
    rsx! {
        fieldset {
            class: format!("fieldset landscape:grid-flow-col bg-base-200 border-base-300 rounded-box w-xs border portrait:mx-auto {}", class.unwrap_or_default()),
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
                    checked: mode() == CipherMode::Ecb,
                    oninput: move |_| {
                        mode.set(CipherMode::Ecb);
                    },
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
                    value: "CBC",
                    checked: mode() == CipherMode::Cbc,
                    oninput: move |_| {
                        mode.set(CipherMode::Cbc);
                    },
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
                    value: "CTR",
                    checked: mode() == CipherMode::Ctr,
                    oninput: move |_| {
                        mode.set(CipherMode::Ctr);
                    },
                }
                "CTR Mode"
            }
        }
    }
}
