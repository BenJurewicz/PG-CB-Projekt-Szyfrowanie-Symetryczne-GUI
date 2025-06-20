use crate::types::{DefaultView, FileData};
use dioxus::prelude::*;

#[component]
pub fn FileDetails(file: FileData, default_view: DefaultView) -> Element {
    let main_view_text = if default_view == DefaultView::PlainText {
        "Text content:"
    } else {
        "Hex View:"
    };
    let second_view_text = if default_view == DefaultView::PlainText {
        "Hex View"
    } else {
        "Text content"
    };

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
                    label {
                        class: "font-semibold text-sm",
                        // "Text content:"
                        "{main_view_text}"
                    }
                    // PlainTextView { file: file.clone() }
                    if default_view == DefaultView::PlainText {
                        PlainTextView { file: file.clone() }
                    } else {
                        HexView { file: file.clone() }
                    }
                }
                details {
                    class: "collapse collapse-arrow bg-base-100 border-base-300 border",
                    summary {
                        class: "collapse-title font-semibold",
                        // "Hex View"
                        "{second_view_text}"
                    }
                    if default_view == DefaultView::PlainText {
                        HexView { file: file.clone() }
                    } else {
                        PlainTextView { file: file.clone() }
                    }
                }

            }
        }
    }
}

#[component]
fn HexView(file: FileData) -> Element {
    rsx! {
        div {
            class: "bg-base-200 rounded-md p-2 mt-1 max-h-60 overflow-y-auto",
            pre { class: "text-wrap text-center text-accent ", "{file.bin_as_hex_string()}" }
        }
    }
}

#[component]
fn PlainTextView(file: FileData) -> Element {
    rsx! {
        div {
            class: "bg-base-200 rounded-md p-2 mt-1 max-h-60 overflow-y-auto",
            pre { class: "text-wrap text-secondary whitespace-pre-wrap", "{file.content_as_string()}" }
        }
    }
}
