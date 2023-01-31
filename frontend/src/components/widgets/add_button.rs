use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct AddButtonProps {
    onclick: Callback<()>,
    title: String,
}

#[function_component(AddButton)]
pub fn add_button(props: &AddButtonProps) -> Html
where
{
    let onclick = {
        let oc = props.onclick.clone();
        move |_| oc.emit(())
    };
    // let modal_visible = use_state(|| false);
    // let onclick = {
    //     let modal_visible = modal_visible.clone();
    //     move |_| modal_visible.set(true)
    // };

    html! {<>
    <button class="button is-link" {onclick}>
      <span class="icon">
      <Icon icon_id={IconId::FontAwesomeRegularSquarePlus}/>
      </span>
      <span>{props.title.clone()}</span>
    </button>
    // if *modal_visible {
    //     // <T on_close={move |_| modal_visible.set(false)}/>
    //     // <T/>
    //     { badger }
    // }
    </>
    }
}
