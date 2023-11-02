use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
      <Box id="welcome-page">
        <H1 id="slogan">"Hello"</H1>

        <H2 id="sub-slogan">"Welcome to Atrium"</H2>
      </Box>
    }
}

