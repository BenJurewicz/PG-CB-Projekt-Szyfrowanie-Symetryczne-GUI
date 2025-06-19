use crate::Route;
use dioxus::prelude::*;

/// Doing:
/// ```
/// // ...
/// enum Route {
///         // ...
///         #[redirect("/", || Route::File {})]
///         #[route("/file")]
///         File {},
///         // ...
/// }
/// ```
/// for some reason gives buggy and inconsistent behavior.
/// Therefore, I made a stub root component that simply redirects to the File route.
/// Perhaps in the future, there could be a full home page with the project description here.
#[component]
pub fn Home() -> Element {
    navigator().replace(Route::File {});
    rsx! {}
}
