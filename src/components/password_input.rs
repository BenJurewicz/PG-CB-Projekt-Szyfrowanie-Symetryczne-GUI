use dioxus::prelude::*;

#[component]
pub fn PasswordInput(password: Signal<String>, class: Option<String>) -> Element {
    rsx! {
        fieldset {
            class: format!("fieldset max-w-1/2 {}", class.unwrap_or_default()),
            legend {
                class: "fieldset-legend py-1.5",
                "Password"
            }
            input {
                type: "text",
                id: "pwd",
                name: "pwd",
                class: "input input-secondary text-base w-full",
                placeholder: "Enter your password",
                oninput: move |e| {
                    password.set(e.value().clone());
                },
                value: password(),
            }
        }
    }
}
