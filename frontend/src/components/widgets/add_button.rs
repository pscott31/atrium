use yew::{html::Buildable, prelude::*};
use yew_icons::{Icon, IconId};

// #[derive(Properties, PartialEq)]
// pub struct AddButtonProps {
//     title: String,
// }

// #[derive(Properties, PartialEq)]
// pub struct MyProps {
//     proppy: String,
// }

// pub trait Closeable {}
#[function_component(AddButton)]
pub fn add_button<T, P, B, TOKEN, How>() -> Html
where
    // T: Component,
    T: Component<Properties = P>,
    P: Properties<Builder = B>,
    B: Buildable<TOKEN, Output = P>,
    B::WrappedToken: yew::html::HasAllProps<P, How>,
    // t::Properties = MyProps>,
    // TP: Properties, // TP: Properties, // T::Properties: Properties, //<Properties: Closable>, // <<T as Component>::Properties as Properties>::on_close: Callback<()>,
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
        <T foo="bar"/>
    }
    </>
    }
}
