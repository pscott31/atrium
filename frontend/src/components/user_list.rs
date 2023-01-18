use crate::components::AddUserButton;
use crate::store::{get_users, User};
use ybc;
use yew::Properties;
use yew::{platform::spawn_local, prelude::*};

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

fn fetch_users(users: UseStateHandle<Vec<User>>) {
    // use crate::components::DbConnection;
    // let db = use_context::<DbConnection>();
    spawn_local(async move {
        let users_vec = get_users().await.expect("failed to get users");
        users.set(users_vec);
    })
}

#[function_component(UserPage)]
pub fn user_page() -> Html {
    // log!("render user page");
    let users = use_state_eq(|| Vec::<User>::new());
    fetch_users(users.clone());

    let arse = (*users).clone();
    html! {
      <UserList users={arse}/>
    }
}
