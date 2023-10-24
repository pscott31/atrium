use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Root default_theme=LeptonicTheme::default()>
        "Content goes here :)"
    </Root>

        <button
            on:click=move |_| {
                set_count(3);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}
