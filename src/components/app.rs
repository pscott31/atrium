use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use crate::store;
use super::{UserPage, UserPageProps, Navbar, NavbarProps, HomePage, HomePageProps, BookingPage, BookingPageProps};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum ConnectionState {
    Ok,
    Pending,
    Failed(store::Error),
}

impl core::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConnectionState::Ok => write!(f, "connected"),
            ConnectionState::Failed(reason) => {
                write!(f, "connection failed: {}", reason.to_string())
            }
            ConnectionState::Pending => write!(f, "connecting"),
        }
    }
}


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let conn_res = create_resource(cx, || (), |_| async move { store::connect().await });

    let conn_state = move || -> ConnectionState {

        match conn_res.read(cx) {
            Some(Ok(_)) => ConnectionState::Ok,
            Some(Err(err)) => ConnectionState::Failed(err),
            None => ConnectionState::Pending,
        }
    };

    create_effect(cx, move |_| {
        log!("connection state changed: {}", conn_state());
    });

    view! { cx,
        <div id="root">
        <Router>
            <Navbar/>
            <section class="section">
                <div class="container">
                    <Show
                        when=move || conn_state() == ConnectionState::Ok
                        fallback=|_| {
                            view! { cx, "loading" }
                        }
                    >
                        <Routes>
                            <Route
                                path="/"
                                view=|cx| {
                                    view! { cx, <HomePage/> }
                                }
                            />
                            <Route
                                path="/users"
                                view=|cx| {
                                    view! { cx, <UserPage/> }
                                }
                            />
                            <Route
                            path="/bookings"
                            view=|cx| {
                                view! { cx, <BookingPage/> }
                            }
                        />
                        </Routes>
                    </Show>
                </div>
            </section>
            <footer>
                <div>"database: " {move || conn_state().to_string()}</div>
            </footer>
        </Router>
        </div>
    }
}
