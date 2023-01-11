use crate::log::log;
use crate::store::User;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(UserAdd)]
pub fn user_add() -> Html {
    log!("rendering  user_add");
    let user = use_state_eq(|| User::default());
    let user2 = user.clone();
    let on_name_update: Callback<String> = (move |s| {
        log!("setting user name to {s}");
        let mut new_user = (*user).clone();
        // new_user.name = s;
        user.set(new_user);
    })
    .into();

    // let name_changed = |s| log!("update {}", s);
    let name_input =
        html! {<ybc::Input name={"name"} value={user2.name.clone()} update={on_name_update}/>};
    html! {
        <>
        <ybc::Field label="Display Name">
          <ybc::Control classes="has-icons-left">
            {name_input}
            <ybc::Icon alignment={ybc::Alignment::Left} size={ybc::Size::Small}><Icon icon_id={IconId::FontAwesomeRegularUser}/></ybc::Icon>
          </ybc::Control>
        </ybc::Field>
        <div>{format!("name: {}", user2.name.clone())}</div>
        </>
    }
}
