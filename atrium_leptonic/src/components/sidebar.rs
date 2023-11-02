use crate::routes::AppRoutes;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn SideBar(
    closed: ReadSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
) -> impl IntoView {
    view! {
      <Drawer id=id shown=Signal::derive(move || !closed.get()) side=DrawerSide::Left>
        <Stack id="nav-stack" orientation=StackOrientation::Vertical spacing=Size::Em(1.0)>
          <NavLink route=AppRoutes::Welcome/>
          <NavLink route=AppRoutes::Doc/>
        </Stack>
      </Drawer>
    }
}

#[component]
pub fn NavLink(route: AppRoutes) -> impl IntoView {
    let icon = move || match route.icon() {
        Some(icon) => Some(view! { <Icon icon=icon/> }),
        None => None,
    };

    view! {
      <Link href=route class="nav-link">
        {icon}
        {route.name()}
      </Link>
    }
}

