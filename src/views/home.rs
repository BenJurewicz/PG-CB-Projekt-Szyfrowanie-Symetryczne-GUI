use dioxus::html::{FileEngine, HasFileData};
use dioxus::prelude::*;
use std::sync::Arc;

struct UploadedFile {
    name: String,
    contents: Vec<u8>,
}

impl UploadedFile {
    fn bin_as_hex_string(&self) -> String {
        self.contents
            .iter()
            .map(|&byte| format!("0x{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn content_as_string(&self) -> String {
        String::from_utf8_lossy(&self.contents).to_string()
    }
}

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        files_uploaded.clear();
        for file_name in &files {
            if let Some(contents) = file_engine.read_file(file_name).await {
                files_uploaded.write().push(UploadedFile {
                    name: file_name.clone(),
                    contents,
                });
            }
        }
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    let handle_download = move |_| {
        // Your binary data
        let binary_data: Vec<u8> = vec![
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21,
        ]; // Example: "Hello World!" in bytes
        let filename = "my_binary_file.bin"; // Appropriate filename extension
        let mime_type = "application/octet-stream"; // Generic binary MIME type

        // Convert Vec<u8> to a base64 string for embedding in JavaScript
        // This is necessary because directly embedding raw bytes in a JS string literal can be problematic
        // for non-UTF-8 characters. Base64 is a safe way to transmit binary data as text.
        let base64_data = base64::encode(&binary_data);

        let js_code = format!(
            r#"
            const base64Data = '{}';
            const filename = '{}';
            const mimeType = '{}';

            // Decode base64 to a Uint8Array
            const binaryString = atob(base64Data);
            const len = binaryString.length;
            const bytes = new Uint8Array(len);
            for (let i = 0; i < len; i++) {{
                bytes[i] = binaryString.charCodeAt(i);
            }}

            const blob = new Blob([bytes], {{ type: mimeType }});
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = filename;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        "#,
            base64_data, filename, mime_type
        );

        document::eval(&js_code);
    };

    rsx! {
        h1 {
            class: "text-4xl font-bold text-center my-8",
        }

        button {
            class: "btn btn-primary ml-4",
            onclick: handle_download,
            "Download File"
        }

        div {
            class: "m-2",
            h1 { "File Upload Example" }
            button { onclick: move |_| files_uploaded.write().clear(), "Clear files" }
            fieldset {
                legend { class: "fieldset-legend", "Upload text/rust files and read them" }
                input {
                    class: "file-input",
                    r#type: "file",
                    onchange: upload_files,
                }
            }

            for file in files_uploaded.read().iter().rev() {
                div {
                    class: "card card-border bg-base-300 m-4 max-w-lg",
                    div {
                        class: "card-body",
                        span { class:"card-title text-primary-content", "{file.name}" }
                        pre  { class: "text-wrap text-secondary", "{file.content_as_string()}"  }
                        pre  { class: "text-wrap text-accent", "{file.bin_as_hex_string()}" }
                    }
                }
            }
        }
    }
}
