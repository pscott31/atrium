use crate::components::UserForm;
use crate::store;
use crate::store::User;

use ybc;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_icons::{Icon, IconId};

#[function_component(AddUserButton)]
pub fn user_add_button() -> Html {
    let modal_visible = use_state(|| false);
    let onclick = {
        let modal_visible = modal_visible.clone();
        move |_| modal_visible.set(true)
    };

    html! {<>
    <button class="button is-link" {onclick}>
      <span class="icon">
      <Icon icon_id={IconId::FontAwesomeRegularSquarePlus}/>
      </span>
      <span>{"Add User"}</span>
    </button>
    <AddUserModal is_visible={*modal_visible} visible_changed={move |v| modal_visible.set(v)}/>
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct UserAddModalProps {
    is_visible: bool,
    visible_changed: Callback<bool>,
}

#[function_component(AddUserModal)]
pub fn user_add_modal(props: &UserAddModalProps) -> Html {
    let user = use_mut_ref(|| User::default());
    let adder = {
        let user = user.clone();
        use_async(async move { store::add_user(&user.borrow()).await })
    };
    let on_update = {
        let user = user.clone();
        move |new_user: User| *user.borrow_mut() = new_user
    };

    let on_save_clicked = {
        let adder = adder.clone();
        move |_| adder.run()
    };

    let vc = props.visible_changed.clone();
    let footer = html! {<>
        <ybc::Button classes="is-success" onclick={on_save_clicked}>{"Save changes"}</ybc::Button>
        <ybc::Button onclick={move |_|vc.emit(false)}>{"Cancel"}</ybc::Button>
        </>
    };

    let on_modal_closed = {
        let vc = props.visible_changed.clone();
        move |_| vc.emit(false)
    };

    // If success from user adder, we can close ourselves
    if let Some(_) = adder.data {
        if props.is_visible {
            props.visible_changed.emit(false)
        }
    }

    // If there was an error, report it and stay open
    let error_box = html! {
        if let Some(error_msg) = &adder.error {
        <div class="notification is-danger">
        {format!("Error adding user: {}", error_msg)}
        </div>
        }
    };

    let body = html! {
    <>
        {error_box}
        <UserForm {on_update}/>
    </>
    };

    html! {
        <crate::components::widgets::Modal title="Add User" {body} {footer} is_active={props.is_visible} on_close={on_modal_closed}/>
    }
}
