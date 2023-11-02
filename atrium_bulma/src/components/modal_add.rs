
use leptos::*;

use super::{Modal, ModalProps};
use crate::store::Addable;

#[component]
pub fn ModalAdd<T, F, G>(
    cx: Scope,
    is_active: RwSignal<bool>,
    default: fn() -> T,
    form: G,
) -> impl IntoView
where
    T: Addable + 'static,
    G: Fn(Scope, RwSignal<T>) -> F + 'static,
    F: IntoView,
{
    let thing = create_rw_signal(cx, default());

    // reset thing to default when activated
    create_effect(cx, move |_| {
        if is_active() {
            thing.set(default())
        }
    });

    let save_action = create_action(cx, move |input: &T| {
        let input = input.clone();
        async move {
            log!("savey wavey");
            let _ = input.add().await;
            is_active.set(false);
            32
        }
    });

    let on_save_clicked = move |_| {
        log!("save pressed");
        save_action.dispatch(thing.get())
    };

    let buttons = move || {
        view! {cx,
            <button class="button is-success" on:click=on_save_clicked>"Save changes"</button>
        }
    };

    view! { cx,
        <Modal is_active=is_active title=format!("Add {}", T::TABLE) buttons=buttons>
            {form(cx, thing)}
        </Modal>
    }
}
