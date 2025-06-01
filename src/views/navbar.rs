use crate::Route;
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
#[component]
pub fn Navbar() -> Element {
    let btn_active_style = "btn-primary pointer-events-none";
    let btn_style = "btn text-xl join-item btn-soft";

    rsx! {
        div {
            class: "navbar bg-base-100 shadow-lg justify-center",
            div {
                class: "navbar-center join",
                // TODO: Probbably this should be a for loop over tuple of routes and vecs
                Link {
                    to: Route::File {},
                    active_class: btn_active_style,
                    class: btn_style,
                    "File"
                }
                Link {
                    to: Route::Text {},
                    active_class: btn_active_style,
                    class: btn_style,
                    "Text"
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
