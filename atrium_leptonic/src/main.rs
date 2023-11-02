mod app;
mod components;
mod pages;
mod routes;

use app::*;
use leptos::*;

fn main() { mount_to_body(|| view! { <App/> }) }

