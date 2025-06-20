use crate::components::{CryptoOptions, ExchangeSVG, FileCard};
use crate::types::{DefaultView, FileData, LastTouched};
use crate::utils::{decrypt_data, encrypt_data, CipherMode};
use dioxus::logger::tracing;
use dioxus::prelude::*;

// Turn decrypted file into encrypted file
fn turn_dec_into_enc_file(password: &str, mode: CipherMode, dec_file: &FileData) -> FileData {
    let data = &dec_file.contents;
    let enc_data = encrypt_data(password, data, mode);
    let file_name = format!("{}.enc", dec_file.name);

    FileData::new(file_name, enc_data)
}

// Turn encrypted file into decrypted file
fn turn_enc_into_dec_file(password: &str, enc_file: &FileData) -> Option<FileData> {
    let data = &enc_file.contents;

    let decrypted = decrypt_data(password, data);
    if let Err(e) = decrypted {
        tracing::error!("Decryption failed: {}", e);
        return None;
    }
    let decrypted = decrypted.unwrap();
    let file_name = enc_file.name.replace(".enc", "");

    Some(FileData::new(file_name, decrypted))
}

#[component]
pub fn File() -> Element {
    let mut decrypted_file: Signal<Option<FileData>> = use_signal(|| None);
    let mut encrypted_file: Signal<Option<FileData>> = use_signal(|| None);

    let mode = use_signal(|| CipherMode::Ctr);
    let password = use_signal(String::new);
    let mut last_touched = use_signal(|| LastTouched::Encrypted);

    use_effect(move || match *last_touched.read() {
        LastTouched::Encrypted => {
            if encrypted_file().is_some() {
                let enc_file = encrypted_file().unwrap();
                let dec_file = turn_enc_into_dec_file(&password(), &enc_file);
                decrypted_file.set(dec_file);
            }
        }
        LastTouched::Decrypted => {
            if decrypted_file().is_some() {
                let dec_file = decrypted_file().unwrap();
                let enc_file = turn_dec_into_enc_file(&password(), mode(), &dec_file);
                encrypted_file.set(Some(enc_file));
            }
        }
    });

    rsx! {
        h1 {
            class: "text-4xl font-bold text-center my-8",
            "File Upload and Download Example"
        }

        p {
            class: "text-center mb-4 portrait:mx-2",
            "Encrypt and decrypt files using AES encryption in different modes."
        }

        CryptoOptions{
            mode: mode,
            password: password,
        }

        div {
            class: "flex lg:flex-row items-center flex-col justify-center gap-4 landscape:mt-25",

            FileCard {
                title: "Decrypted File",
                file: decrypted_file,
                default_view: DefaultView::PlainText,
                oninput: move |_| {
                    last_touched.set(LastTouched::Decrypted)
                }
            }

            button {
                class: "btn btn-ghost h-20",
                ExchangeSVG {
                    class: "max-lg:rotate-90"
                }
            }

            FileCard {
                title: "Encrypted File",
                file: encrypted_file,
                default_view: DefaultView::Hex,
                oninput: move |_| {
                    last_touched.set(LastTouched::Encrypted)
                }
            }

        }
    }
}
