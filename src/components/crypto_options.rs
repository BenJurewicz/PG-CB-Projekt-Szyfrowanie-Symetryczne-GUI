use crate::components::{CryptoModeSelect, PasswordInput};
use crate::utils::CipherMode;
use dioxus::prelude::*;

#[component]
pub fn CryptoOptions(mode: Signal<CipherMode>, password: Signal<String>) -> Element {
    rsx! {
        div {
            class: "flex portrait:flex-col landscape:gap-15 portrait:gap-4 justify-center portrait:items-center mb-4",
            CryptoModeSelect {
                mode: mode,
            }
            PasswordInput{
                password: password,
            }
        }
    }
}
