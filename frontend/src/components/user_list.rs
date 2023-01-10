use crate::store::{get_users, User};
use ybc;
use yew::{platform::spawn_local, prelude::*};

// fn dave(users: &Vec<User>) -> Html {
//     html! {
//         <table>{
//         for users.iter().map(|user| {
//           <tr>
//             <td>{ user.name }</td>
//             <td>{ user.contact_name }</td>
//             <td>{ user.email }</td>
//             <td>{ user.phone }</td>
//           </tr>
//         })
//       }
//       </table>
//     }
// }

#[function_component(UserList)]
pub fn user_list() -> Html {
    let users = use_state(|| Vec::<User>::new());
    fetch_users(users.clone());

    html! {
      <>
        <ybc::Table>
        {for users.iter().map(|user| html!{
          <tr>
            <td>{ user.name.clone() }</td>
            <td>{ user.contact_name.clone() }</td>
            <td>{ user.email.clone() }</td>
            <td>{ user.phone.clone() }</td>
          </tr>
        })}


          //   <tr>
          //     <td>{"foo"}</td>
          //     <td>{"bar"}</td>
          //   </tr>
          //   <tr>
          //   <td>{"jim"}</td>
          //   <td>{"bob"}</td>
          // </tr>
          // <tr>
          // <td>{"fred"}</td>
          // <td>{"bloggs"}</td>
        // </tr>
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
