use leptos::*;

use super::{ModalAdd, ModalAddProps, UserForm, UserFormProps, UserList, UserListProps};
use crate::store::User;

#[component]
pub fn UserPage(cx: Scope) -> impl IntoView {
    let is_active = create_rw_signal(cx, false);
    let add_form = |cx: Scope, user| view! { cx, <UserForm thing=user/> };

    let arse = move |_| is_active.set(true);
    view! { cx,
        <h1 class="title">"Users"</h1>
        <UserList/>
        <ModalAdd is_active=is_active default=User::default form=add_form/>
        <div class="buttons">
            <button class="button is-primary" on:click=arse>
                "Add User"
            </button>
        </div>
    }
}
