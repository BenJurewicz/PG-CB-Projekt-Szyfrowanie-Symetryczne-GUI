//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod file_details;
use file_details::FileDetails;

mod crypto_options;
pub use crypto_options::CryptoOptions;

mod exchange_svg;
pub use exchange_svg::ExchangeSVG;

mod crypto_mode_select;
pub use crypto_mode_select::CryptoModeSelect;

mod file_card;
pub use file_card::FileCard;

mod textarea;
pub use textarea::Textarea;

mod password_input;
pub use password_input::PasswordInput;
