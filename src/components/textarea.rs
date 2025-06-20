use dioxus::prelude::*;

#[component]
// When passing a oniput callback, the text signal is NOT updated.
pub fn Textarea(
    textarea_id: String,
    title: String,
    placeholder: String,
    text: Signal<String>,
    class: Option<String>,
    textarea_class: Option<String>,
    oninput: Option<Callback<Event<FormData>, ()>>,
    onclick: Option<Callback>,
) -> Element {
    rsx! {
        div {
            onclick: move |_| {
                if let Some(c) = onclick {
                    c.call(());
                }
            },

            class: format!("flex flex-col w-full landscape:w-1/2 max-w-[530px] {}",
                class.unwrap_or_default()),

            label {
                for: textarea_id.clone(),
                class: "mb-1 text-primary-content",
                "{title}"
            }

            textarea {
                id: textarea_id.clone(),
                class: format!("textarea textarea-bordered w-full font-mono text-base {}",
                    textarea_class.unwrap_or_default()),
                placeholder: placeholder,
                rows: 10,

                oninput: move |e| {
                    match &oninput {
                        Some(cb) => cb.call(e),
                        None => text.set(e.value().clone()),
                    }
                },
                value: text(),
            }
        }
    }
}
