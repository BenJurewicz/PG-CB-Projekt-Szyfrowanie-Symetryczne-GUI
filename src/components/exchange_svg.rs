use crate::EXCHANGE_SVG;
use dioxus::prelude::*;

#[component]
pub fn ExchangeSVG(class: Option<String>) -> Element {
    rsx! {
        img {
            src: "{EXCHANGE_SVG}",
            alt: "Exchange Icon",
            class: format!("w-8 h-8 portrait:rotate-90 {}", class.unwrap_or_default()),
        }
    }
}
