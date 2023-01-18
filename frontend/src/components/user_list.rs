use crate::components::AddUserButton;
use crate::state::GlobalState;
use crate::store::{get_users, User};
use ybc;
use yew::Properties;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_effect_once;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserListProps {
    pub users: Vec<User>,
}

#[function_component(UserList)]
pub fn user_list(UserListProps { users }: &UserListProps) -> Html {
    html! {
      <>
        <AddUserButton/>
        <ybc::Table>
          <thead>
            <tr>
              <th>{"Name"}</th>
              <th>{"Contact"}</th>
              <th>{"Contact Email"}</th>
              <th>{"Contact Phone"}</th>
            </tr>
          </thead>
          <tbody>
            {for users.iter().map(|user| html!{
            <tr>
              <td>{ user.name.clone() }</td>
              <td>{ user.contact_name.clone() }</td>
              <td>{ user.email.clone() }</td>
              <td>{ user.phone.clone() }</td>
            </tr>
          })}
          </tbody>
        </ybc::Table>
        </>
    }
}

fn fetch_users() {
    // use crate::components::DbConnection;
    // let db = use_context::<DbConnection>();
    spawn_local(async move {
        let users_vec = get_users().await.expect("failed to get users");
        Dispatch::<GlobalState>::new().reduce_mut(|gs| gs.users = users_vec);
    })
}

#[function_component(UserPage)]
pub fn user_page() -> Html {
    use_effect_once(|| {
        fetch_users();
        || ()
    });
    // log!("render user page");
    // let users = use_state_eq(|| Vec::<User>::new());
    let (gs, _) = use_store::<GlobalState>();
    html! {
      //todo maybe don't pass this guy in, just get global state in there
      <UserList users={gs.users.clone()}/>
    }
}
