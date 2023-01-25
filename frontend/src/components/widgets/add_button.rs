use yew::{html::Buildable, prelude::*};
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct AddButtonProps {
    title: String,
}

pub struct MyToken {}

pub trait Closeable {}
#[function_component(AddButton)]
pub fn add_button<T>() -> Html
where
    T: Component,
    // T::Properties: Properties, //<Properties: Closable>, // <<T as Component>::Properties as Properties>::on_close: Callback<()>,
    // <T::Properties as Properties>::Builder: Buildable<MyToken>,
{
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
    if *modal_visible {
        // <T on_close={move |_| modal_visible.set(false)}/>
        <T />
    }
    </>
    }
}
