use leptos::*;
use crate::store::{get_users, User};
use super::{UserForm, UserFormProps, ModalEdit, ModalEditProps};

#[component]
pub fn UserList(cx: Scope) -> impl IntoView {
    let user_res = create_resource(
        cx,
        || (),
        move |_| async move { get_users().await.unwrap() },
    );

    let users = move || match user_res.read(cx) {
        Some(value) => value,
        None => Vec::<User>::new(),
    };

    let is_active = create_rw_signal(cx, false);
    let edit_form = |cx: Scope, user| view! { cx, <UserForm thing=user/> };
    let editing_user = create_rw_signal(cx, User::default());

    let edit_clicked = move |u: RwSignal<User>| {
        move |_| {
            editing_user.set(u());
            is_active.set(true);
        }
    };

    view! { cx,
        <ModalEdit is_active=is_active thing=editing_user form=edit_form/>
        <table class="table">
            <thead>
                <tr>
                    <th>"Name"</th>
                    <th>"Contact Name"</th>
                    <th>"Email"</th>
                    <th>"Phone"</th>
                    <th/>
                </tr>
            </thead>
            <For
                each=users
                key=|key| key.name.clone()
                view=move |cx, user| {
                    let u = create_rw_signal(cx, user);
                    view! { cx,
                        <tr>
                            <td>{u().name}</td>
                            <td>{u().contact_name}</td>
                            <td>{u().email}</td>
                            <td>{u().phone}</td>
                            <td>
                                <button class="button is-primary is-small"
                                 on:click=edit_clicked(u)
                                >
                                    "Edit"
                                </button>
                            </td>
                        </tr>
                    }
                }
            />
        </table>
    }
}
