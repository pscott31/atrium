use crate::components::UserForm;
use crate::log::log;
use crate::store;
use crate::store::User;

use ybc;
use yew::prelude::*;
use yew_hooks::use_async;
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

    let on_update = {
        let user = user.clone();
        move |new_user: User| {
            log!("updated user: {user:?}");
            *user.borrow_mut() = new_user
        }
    };

    let add_user = use_async(async move { store::add_user(&user.borrow()).await });
    let close_ctx = use_context::<ybc::ModalCloserContext>().expect("no close ctx found");

    let footer = html! {<>
        <ybc::Button classes="is-success" onclick={move|_| add_user.run()}>{"Save changes"}</ybc::Button>
        <ybc::Button >{"Cancel"}</ybc::Button>
        </>
    };

    let body = html! {<UserForm {on_update}/>};
    let trigger = props.trigger.clone();

    html! {
        <ybc::ModalCard id="cake" title="Add User"  {trigger} {body} {footer}/>
    }
}
