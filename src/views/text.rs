use dioxus::prelude::*;

#[component]
pub fn Text() -> Element {
    rsx! {
        h1 {
            class: "text-4xl font-bold text-center my-8",
            "Text Encryption"
        }

        p {
            class: "text-center mb-4",
            "Encrypt and decrypt text using AES encryption in different modes."
        }
        div {
            p {
                class: "text-center mb-4",
                "Select an encryption mode and enter text to encrypt."
            }
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
                class: "label cursor-pointer",
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
                class: "label cursor-pointer",
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
                class: "label cursor-pointer",
                "CTR Mode"
            }
        }



        textarea {
            class: "textarea textarea-bordered w-full max-w-lg mx-auto",
            placeholder: "Enter text to encrypt...",
            rows: 10,
        }

        textarea {
            class: "textarea textarea-bordered w-full max-w-lg mx-auto mt-4",
            placeholder: "Encrypted text will appear here...",
            rows: 10,
        }
    }
}
