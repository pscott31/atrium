use leptos::*;

use super::{ErrorMsg, ErrorMsgProps, Modal, ModalProps};
use crate::store::Updatable;

#[component]
pub fn ModalEdit<T, F, G>(
    cx: Scope,
    is_active: RwSignal<bool>,
    thing: RwSignal<T>,
    form: G,
) -> impl IntoView
where
    T: Updatable + Default + Clone + std::fmt::Debug + 'static,
    G: Fn(Scope, RwSignal<T>) -> F + 'static,
    F: IntoView,
{
    let error = create_rw_signal(cx, None);

    let update_action = create_action(cx, move |input: &T| {
        let input = input.clone();
        async move {
            log!("update away! {:?}", input);
            match input.update().await {
                    Err(err) => {
                    log!("SETTING ERROR {:?}", err);
                    error.set(Some(err.to_string()))},
                Ok(_) => is_active.set(false)
            }
            // let _ = input.add().await;
            ;
            32
        }
    });

    let on_save_clicked = move |_| {
        log!("save pressed");
        update_action.dispatch(thing.get())
    };

    let buttons = move || {
        view! { cx,
            <button class="button is-success" on:click=on_save_clicked>
                "Save changes"
            </button>
        }
    };

    view! { cx,
        <Modal is_active=is_active title=format!("Edit {}", T::TABLE)  buttons=buttons>
        <ErrorMsg title="Error updating user" error=error/>
            {form(cx, thing)}
        </Modal>
    }
}
