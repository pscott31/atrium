use crate::components::UserAdd;
use crate::log::log;
use crate::store::{get_users, User};
use ybc;
use yew::Properties;
use yew::{platform::spawn_local, prelude::*};
use yew_icons::{Icon, IconId};
#[derive(Properties, PartialEq)]
pub struct UserListProps {
    pub users: Vec<User>,
}

fn add_user() {
    log!("clicky")
}

// onclick={Callback::from(|_| (add_user()))}
#[function_component(UserList)]
pub fn user_list(UserListProps { users }: &UserListProps) -> Html {
    log!("refresh_user_list");

    let add_button = html! {
    <button class="button is-link">
      <span class="icon">
      <Icon icon_id={IconId::FontAwesomeRegularSquarePlus}/>
      </span>
      <span>{"Add User"}</span>
    </button>
    };

    html! {
      <>
        <ybc::ModalCard id="cake" title="Add User" trigger={add_button} body={html!{<UserAdd/>}}/>
        // {add_button}
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
    spawn_local(async move {
        let users_vec = get_users().await.expect("failed to get users");
        users.set(users_vec);
    })
}

#[function_component(UserPage)]
pub fn user_page() -> Html {
    log!("render user page");
    let users = use_state_eq(|| Vec::<User>::new());
    fetch_users(users.clone());

    let arse = (*users).clone();
    html! {
      <UserList users={arse}/>
    }
}
