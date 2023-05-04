use leptos::*;

#[component]
pub fn ErrorMsg(
    cx: Scope,
    #[prop(into)] error: Signal<Option<String>>,
    #[prop(into)] title: MaybeSignal<String>,
) -> impl IntoView {
    view! { cx,
        {move || match error.get() {
            Some(error_text) => {
                view! { cx,
                    <article class="message is-danger">
                        <div class="message-header">
                            <p>{title.get()}</p>
                        </div>
                        <div class="message-body">{error_text}</div>
                    </article>
                }
                    .into_view(cx)
            }
            None => {
                view! { cx,  }
                    .into_view(cx)
            }
        }}
    }
}
