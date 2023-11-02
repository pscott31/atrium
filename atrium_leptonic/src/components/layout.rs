use crate::components::*;
use leptonic::prelude::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    let (drawer_closed, _set_drawer_closed) = create_signal(false);

    view! {
      <TopBar/>
      <div>yay</div>
      <Box id="layout-lower">
        <SideBar id="layout-sidebar" closed=drawer_closed/>
        <Box id="layout-content">
          <Outlet/>
        </Box>
      </Box>
    }
}

