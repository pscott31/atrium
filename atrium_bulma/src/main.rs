mod log;
mod store;
mod components;

use components::{App, AppProps};
use leptos::*;

extern crate console_error_panic_hook;
use std::panic;


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(|cx| view! { cx, <App/> })
}
