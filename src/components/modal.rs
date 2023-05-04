use leptos::*;

#[component]
pub fn Modal<F, IV>(
    cx: Scope,
    is_active: RwSignal<bool>,
    buttons: F,
    title: &'static str,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let deactivate = move |_| is_active.set(false);

    view! { cx,
        <div class="modal" class:is-active=is_active>
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{title}</p>
                    <button class="delete"  on:click=deactivate></button>
                </header>
                <section class="modal-card-body">{children(cx)}</section>
                <footer class="modal-card-foot">
                    {buttons()}
                    <button class="button" on:click=deactivate>
                        "Cancel"
                    </button>
                </footer>
            </div>
        </div>
    }
}
