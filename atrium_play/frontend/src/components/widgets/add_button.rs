use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct AddButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub title: String,
}

#[function_component(AddButton)]
pub fn add_button(props: &AddButtonProps) -> Html
where
{
    html! {
    <button class="button is-link" onclick={props.onclick.clone()}>
      <span class="icon">
      <Icon icon_id={IconId::FontAwesomeRegularSquarePlus}/>
      </span>
      <span>{props.title.clone()}</span>
    </button>
    }
}
