use crate::components::UserForm;
use crate::log::log;
use crate::store::User;

use ybc;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(AddUserButton)]
pub fn user_add_button() -> Html {
    let trigger = html! {
    <button class="button is-link">
      <span class="icon">
      <Icon icon_id={IconId::FontAwesomeRegularSquarePlus}/>
      </span>
      <span>{"Add User"}</span>
    </button>
    };

    html! {
      <AddUserModal {trigger}/>
    }
}

#[derive(Properties, PartialEq)]
pub struct UserAddModalProps {
    trigger: Html,
}

#[function_component(AddUserModal)]
pub fn user_add_modal(props: &UserAddModalProps) -> Html {
    let user = use_mut_ref(|| User::default());

    let onupdate = {
        let user = user.clone();
        move |new_user: User| {
            log!("updated user: {user:?}");
            *user.borrow_mut() = new_user
        }
    };

    let add_user = move |_| {
        log!("saving new user: {:?}", user);
    };

    let footer = html! {
      <>
      <ybc::Button classes="is-success" onclick={add_user}>{"Save changes"}</ybc::Button>
      <ybc::Button >{"Cancel"}</ybc::Button>
      </>
    };

    let body = html! {<UserForm {onupdate}/>};
    let trigger = props.trigger.clone();

    html! {
        <ybc::ModalCard id="cake" title="Add User"  {trigger} {body} {footer}/>
    }
}
