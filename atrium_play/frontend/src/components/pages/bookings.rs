use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_effect_once;
use yewdux::prelude::{use_store, Dispatch};

use crate::{
    components::{widgets::AddButton, AddModal, HasForm},
    state::GlobalState,
    store::{get_bookings, Booking},
};

//////////////////////////////////////////////// page
#[function_component(BookingsPage)]
pub fn bookings_page() -> Html {
    use_effect_once(|| {
        fetch_bookings();
        || ()
    });
    let (gs, _) = use_store::<GlobalState>();
    html! {
      //todo maybe don't pass this guy in, just get global state in there
      <BookingList bookings={gs.bookings.clone()}/>
    }
}

fn fetch_bookings() {
    spawn_local(async move {
        let res = get_bookings().await.expect("failed to get bookings");
        Dispatch::<GlobalState>::new().reduce_mut(|gs| gs.bookings = res);
    })
}

//////////////////////////////////////////////// list

#[derive(Properties, PartialEq)]
pub struct BookingListProps {
    pub bookings: Vec<Booking>,
}

#[function_component(BookingList)]
pub fn user_list(BookingListProps { bookings }: &BookingListProps) -> Html {
    let add_modal_visible = use_state(|| false);
    //TODO
    let dave = add_modal_visible.clone();
    let frank = add_modal_visible.clone();
    html! {
      <>
        <AddButton title={"New Booking"} onclick={move |_| add_modal_visible.set(true)}/>
        <AddModal<Booking> visible={*dave} on_close={move |()| frank.set(false)}/>
        <ybc::Table>
          <thead>
            <tr>
              <th>{"User"}</th>
              <th>{"Start"}</th>
              <th>{"Duration"}</th>
            </tr>
          </thead>
          <tbody>
            {for bookings.iter().map(|booking| html!{
            <tr>
              <td>{ booking.user.clone() }</td>
              <td>{ format!("{:?}", booking.start_time.clone())}</td>
              <td>{ format!("{:?}", booking.duration.clone() )}</td>
            </tr>
          })}
          </tbody>
        </ybc::Table>
        </>
    }
}

//////////////////////////////////////////////// hasform
///
impl HasForm for Booking {
    fn form<C>(ctx: &Context<C>) -> Html
    where
        C: Component<Message = Msg<User>>,
    {
        html! {
            <UserForm on_update={ctx.link().callback(|u| Msg::Updated(u))} />
        }
    }
}
