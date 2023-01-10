use crate::store::{get_users, User};
use ybc;
use yew::{platform::spawn_local, prelude::*};

#[function_component(UserList)]
pub fn user_list() -> Html {
    let users = use_state(|| Vec::<User>::new());
    fetch_users(users.clone());

    html! {
      <>
      <div>{format!("{:?}", users)}</div>
        <ybc::Table>
            <tr>
              <td>{"foo"}</td>
              <td>{"bar"}</td>
            </tr>
            <tr>
            <td>{"jim"}</td>
            <td>{"bob"}</td>
          </tr>
          <tr>
          <td>{"fred"}</td>
          <td>{"bloggs"}</td>
        </tr>
        </ybc::Table>
        </>
    }
}

fn fetch_users(users: UseStateHandle<Vec<User>>) {
    spawn_local(async move {
        let users_vec = get_users().await.expect("failed to get users");
        users.set(users_vec);
    })
}
