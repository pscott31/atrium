use crate::components;
use crate::enclose::enclose;
use crate::log::wasm_log;
use crate::store;
use futures::future::TryFutureExt;
use ybc::NavbarFixed::Top;
use ybc::NavbarItemTag::A;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let connect = use_async(store::connect().map_err(|x| format!("Error connecting: {}", x)));

    use_effect_once(enclose! { (connect) move || {connect.run(); ||() }});

    match &(*connect) {
        UseAsyncState { loading: true, .. } => loading(),
        UseAsyncState { error: Some(e), .. } => error_box(e),
        UseAsyncState { data: Some(_), .. } => main_stuff(),
        _ => error_box("Internal Error"),
    }
}

fn loading() -> Html {
    html! {
      <div class="center-screen">
        <div class="lds-ellipsis"><div></div><div></div><div></div><div></div></div>
      </div>
    }
}

fn error_box(error: &str) -> Html {
    wasm_log("error_box");
    html! {
      <div class="container">
        <article class="message is-danger">
        <div class="message-header">
            <p>{"Unable to connect to database"}</p>
        </div>
        <div class="message-body">
            {error}
        </div>
        </article>
      </div>
    }
}

fn main_stuff() -> Html {
    html! {
      <>
        <ybc::Navbar fixed={Top} navstart={navbar_items()}></ybc::Navbar>
        <ybc::Section>
          <ybc::Container fluid=true>
            <components::BookingsPage/>
          </ybc::Container>
        </ybc::Section>

        <ybc::Section>
          <ybc::Container fluid=true>
            <components::UserPage/>
          </ybc::Container>
        </ybc::Section>
      </>
    }
}

fn navbar_items() -> Html {
    html! {
        <>
            <ybc::NavbarItem tag={A} href="/">
                { "Home" }
            </ybc::NavbarItem>
            <ybc::NavbarItem tag={A} href="https://github.com/anlumo/tttod/blob/main/README.md">
                { "About" }
            </ybc::NavbarItem>
        </>
    }
}
