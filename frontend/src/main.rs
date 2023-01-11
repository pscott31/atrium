use yew::prelude::*;

use frontend::components;
use frontend::log::wasm_log;
use ybc::NavbarFixed::Top;
use ybc::NavbarItemTag::A;

#[function_component(App)]
fn app() -> Html {
    wasm_log("refresh_app");
    html! {
      <>
      <ybc::Navbar fixed={Top} navstart={navbar_items()}></ybc::Navbar>
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

fn main() {
    yew::Renderer::<App>::new().render();
}
