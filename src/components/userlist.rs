use leptos::*;
use crate::store::{get_users, User};

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

    view! { cx,
        <table class="table">
            <thead>
                <tr>
                    <th>"Name"</th>
                    <th>"Contact Name"</th>
                    <th>"Email"</th>
                    <th>"Phone"</th>
                </tr>
            </thead>
            <For
                each=users
                key=|u| u.name.clone()
                view=move |cx, cake| {
                    view! { cx,
                        <tr>
                            <td>{cake.name}</td>
                            <td>{cake.contact_name}</td>
                            <td>{cake.email}</td>
                            <td>{cake.phone}</td>
                        </tr>
                    }
                }
            />
        </table>

    }
}
