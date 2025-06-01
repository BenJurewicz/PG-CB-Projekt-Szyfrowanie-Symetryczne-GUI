use crate::types::FileData;
use dioxus::prelude::*;

#[component]
pub fn FileDetails(file: FileData) -> Element {
    rsx! {
        div {
            class: "card card-border bg-base-300 m-4 max-w-full md:max-w-2xl",
            div {
                class: "card-body p-4",
                span { class: "card-title text-primary-content flex justify-between items-center",
                    "{file.name}"
                    span { class: "text-xs text-gray-500", "{file.contents.len()} bytes" }
                }

                div {
                    class: "mt-2",
                    label { class: "font-semibold text-sm", "Text content:" }
                    div {
                        class: "bg-base-200 rounded-md p-2 mt-1 max-h-60 overflow-y-auto",
                        pre { class: "text-wrap text-secondary whitespace-pre-wrap", "{file.content_as_string()}" }
                    }
                }

                details {
                    class: "collapse collapse-arrow bg-base-100 border-base-300 border",
                    summary { class: "collapse-title font-semibold", "Hex View" }
                    div {
                        class: "bg-base-200 rounded-md p-2 mt-1 max-h-60 overflow-y-auto",
                        pre { class: "text-wrap text-accent break-all", "{file.bin_as_hex_string()}" }
                    }
                }
            }
        }
    }
}
