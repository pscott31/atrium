use crate::routes::AppRoutes;
use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;

use crate::components::*;
use crate::pages::{err404::PageErr404, welcome::PageWelcome};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Title text="Atrium"/>
      <Root default_theme=LeptonicTheme::default()>
        <Router>
          <Routes>
            <Route path="" view=|| view! { <Layout/> }>
              <Route path=AppRoutes::Welcome view=|| view! { <PageWelcome/> }/>
              <Route path=AppRoutes::NotFound view=|| view! { <PageErr404/> }/>
            </Route>
          </Routes>
        </Router>
      </Root>
    }
}

