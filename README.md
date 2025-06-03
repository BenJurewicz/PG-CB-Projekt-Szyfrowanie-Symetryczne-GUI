# Symmetric Encryption

This is a Dioxus app that demonstrates diffrent symmetric encryption modes (ECB, CBC, CTR) using the AES algorithm.
It allows users to encrypt and decrypt files and text using a password.

## Tech Stack

- Lang: **Rust**
- GUI Framework **Dioxus**: Rust framework similar to React, used for building cross-platform applications.
- Styling: **Tailwind CSS**, **DaisyUI**
- Encryption related libraries:
    - **aes**: Advanced Encryption Standard for symmetric encryption.
    - **ecb, cbc, ctr**: Crates implementing different modes of AES encryption.
    - **PBKDF2**: Password-Based Key Derivation Function 2 for securely deriving keys from passwords.
    - **sha2**: Neded for SHA-256 hashing, used in PBKDF2.
- Other:
    - **RFD**: Cross-platform file dialog library.

# Setup Instructions for Development

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install)
2. [Dioxus](https://dioxuslabs.com/learn/0.6/getting_started/#) (Specifically, the Dioxus CLI)
3. [Node.js](https://nodejs.org/en/download/) (for Tailwind CSS)

## Project Setup

1. Clone the repository
2. In the project root, run the following command to install necessary dependencies:

```bash
npm install
```

(Cargo Dependencies will be installed automatically when you run the app for the first time)

## Development

Run the following commands in the root of the project:

-
    1. Start the Tailwind CSS watcher to automatically recompile styles when changes are made:
       ```bash
       npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --watch
       ```
-
    2. Start the Dioxus development server:

    - For Web:
      ```bash
      dx serve --platform web --hot-reload true
      ```

    - For Desktop:
      ```bash
      dx serve --platform desktop --hot-reload true
      ```

## Building for Production

### For the Web:

Deploy to the web:
**Remember to uncomment the line `base_path = "/szyfrowanie-symetryczne"` in `Dioxus.toml` file** \

```bash
npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css
```

```bash
dx bundle --platform web --out-dir web-app
```

### For Desktop:

<!-- TODO: Double check if everything is correct in the desktop sections -->
**Remember to make sure the line `base_path = "/szyfrowanie-symetryczne"` in `Dioxus.toml` file \
is commented out.**
\

```bash
npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css
```

```bash
dx bundle --platform desktop --out-dir desktop-app
```
