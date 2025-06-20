use crate::components::{CryptoOptions, ExchangeSVG, Textarea};
use crate::types::LastTouched;
use crate::utils::{decrypt_data, encrypt_data, hex_to_vec, vec_to_hex, CipherMode};
use dioxus::logger::tracing;
use dioxus::prelude::*;

// Turn decrypted text into encrypted hex string
fn turn_dec_into_enc(
    password: &str,
    mode: CipherMode,
    dec_text: &str,
    mut invalid_encryption: Signal<bool>,
) -> String {
    invalid_encryption.set(false);
    let text = dec_text.as_bytes().to_vec();
    let enc_text = encrypt_data(password, &text, mode);
    vec_to_hex(&enc_text)
}

// Turn encrypted hex string into decrypted text
fn turn_enc_into_dec(
    password: &str,
    enc_text: &str,
    mut invalid_encryption: Signal<bool>,
) -> String {
    let text = hex_to_vec(enc_text).map_err(|e| e.to_string());
    if let Err(err) = text {
        tracing::error!("Hex decode failed: {}", err);
        invalid_encryption.set(true);
        return String::new();
    }
    let text = text.unwrap();

    let decrypted = decrypt_data(password, &text);
    if let Err(e) = decrypted {
        tracing::error!("Decryption failed: {}", e);
        invalid_encryption.set(true);
        return String::new();
    }
    let decrypted = decrypted.unwrap();
    invalid_encryption.set(false);
    String::from_utf8_lossy(&decrypted).to_string()
}

#[component]
pub fn Text() -> Element {
    let mode = use_signal(|| CipherMode::Ctr);
    let password = use_signal(String::new);
    let mut encrypted_text = use_signal(String::new);
    let mut decrypted_text = use_signal(String::new);
    let mut last_touched = use_signal(|| LastTouched::Decrypted);
    let invalid_encryption = use_signal(|| false);

    use_effect(move || match *last_touched.read() {
        LastTouched::Encrypted => {
            if encrypted_text().is_empty() {
                decrypted_text.set(String::new());
            } else {
                decrypted_text.set(turn_enc_into_dec(
                    &password(),
                    &encrypted_text(),
                    invalid_encryption,
                ));
            }
        }
        LastTouched::Decrypted => {
            if decrypted_text().is_empty() {
                encrypted_text.set(String::new());
            } else {
                encrypted_text.set(turn_dec_into_enc(
                    &password(),
                    mode(),
                    &decrypted_text(),
                    invalid_encryption,
                ));
            }
        }
    });

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

            CryptoOptions{
                mode: mode,
                password: password,
            }

            div {
                class: "flex flex-row items-center justify-between w-full landscape:mt-15 portrait:!flex-col portrait:!justify-center portrait:!items-center landscape:mb-10 portrait:mb-5",

                Textarea {
                    textarea_class: if invalid_encryption() {
                        "border border-error"
                    } else if last_touched() == LastTouched::Decrypted{
                        "border border-secondary"
                    },
                    class: if invalid_encryption() {
                        "text-error"
                    },
                    textarea_id: "encrypted-text",
                    title: "Text To Encrypt:",
                    placeholder: if invalid_encryption() {
                        "Error on decryption attempt, please check your password or input."
                    } else {"Enter text to encrypt..." },
                    text: decrypted_text,
                    onclick: move |_| {
                        last_touched.set(LastTouched::Decrypted);
                    },
                    oninput: move |e: Event<FormData>| {
                        last_touched.set(LastTouched::Decrypted);

                        let text = e.value().clone();
                        decrypted_text.set(text);
                    },
                }

                ExchangeSVG {
                    class: "m-10"
                }

                Textarea {
                    textarea_class: if invalid_encryption() {
                        "border border-error"
                    } else if last_touched() == LastTouched::Encrypted {
                        "border border-secondary"
                    },
                    class: if invalid_encryption() {
                        "text-error"
                    },
                    textarea_id: "decrypt-text",
                    title: "Text To Decrypt:",
                    placeholder: "Enter text to decrypt here...",
                    text: encrypted_text,
                    onclick: move |_| {
                        last_touched.set(LastTouched::Encrypted);
                    },
                    oninput: move |e: Event<FormData>| {
                        last_touched.set(LastTouched::Encrypted);

                        let text = e.value().clone();
                        encrypted_text.set(text);
                    },
                }
            }
        }
    }
}
