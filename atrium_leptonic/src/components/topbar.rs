use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
      <AppBar>
        <H3 style="margin-left: 1em">"Atrium"</H3>
        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
          <Icon icon=BsIcon::BsGithub/>
          <Icon icon=BsIcon::BsPower/>
        </Stack>
      </AppBar>
    }
}

