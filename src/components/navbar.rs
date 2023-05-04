use leptos::*;

extern crate console_error_panic_hook;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="navbar">
            <div class="navbar-brand">
                <a class="navbar-item">
                    <img src="logo.png"/>
                </a>
                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>
            <div class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item">"Home"</a>
                    <a class="navbar-item">"Users"</a>
                </div>
            </div>
        </nav>
    }}