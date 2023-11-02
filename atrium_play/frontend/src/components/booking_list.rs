// use yew::prelude::*;
// use yew_icons::{Icon, IconId};

// #[derive(Properties, PartialEq)]
// pub struct AddModalProps {
//     is_visible: bool,
//     visible_changed: Callback<bool>,
// }

// #[function_component(AddButton)]
// pub fn add_button<T>() -> Html
// where
//     T: Component<Properties = AddModalProps>,
//     // T::Properties::Builder: Buildable,
//     // <<T as yew::Component>::Properties>::Builder: ModalPropsBuilder,
// {
//     let modal_visible = use_state(|| false);
//     let onclick = {
//         let modal_visible = modal_visible.clone();
//         move |_| modal_visible.set(true)
//     };

//     html! {<>
//     <button class="button is-link" {onclick}>
//       <span class="icon">
//       <Icon icon_id={IconId::FontAwesomeRegularSquarePlus}/>
//       </span>
//       <span>{"Add User"}</span>
//     </button>
//     <T is_visible={*modal_visible} visible_changed={move |v| modal_visible.set(v)}/>
//     </>
//     }
// }

// #[function_component(BookingList)]
// pub fn booking_list() -> Html {
//     html! {
//     <>
//       <h1>{"Booking List"}</h1>
//     </>}
// }
