use crate::Route;
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "navbar bg-base-100 shadow-lg justify-center",
            div {
                class: "navbar-center",
                Link {
                    to: Route::Home {},
                    class: "btn btn-ghost text-xl",
                    "Home"
                }
                Link {
                    to: Route::Blog { id: 1 },
                    class: "btn btn-ghost text-xl",
                    "Blog"
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
