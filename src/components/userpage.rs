use leptos::*;

use super::{UserAdd, UserAddProps, UserList, UserListProps, ModalAdd, ModalAddProps};
use crate::store::User;

#[component]
pub fn UserPage(cx: Scope) -> impl IntoView {
    let is_active = create_rw_signal(cx, false);
    // let deactivate = move |_| set_is_active(false);
    let activate = move |_| is_active.set(true);

    let on_save = move |u:User| log!("save: {:?}", u);

    view! { cx,
        <h1 class="title">"Users"</h1>
        <UserList/>
        <ModalAdd is_active=is_active title="Add User"><UserAdd/></ModalAdd>
        <div class="buttons">
            <button class="button is-primary" on:click=activate>
                "Add User"
            </button>
        </div>
    }
}
