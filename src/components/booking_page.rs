use leptos::*;

use super::{ModalAdd, ModalAddProps, BookingForm, BookingFormProps, BookingList, BookingListProps};
use crate::store::Booking;

#[component]
pub fn BookingPage(cx: Scope) -> impl IntoView {
    let is_active = create_rw_signal(cx, false);
    let add_form = |cx: Scope, user| view! { cx, <BookingForm thing=user/> };

    let arse = move |_| is_active.set(true);
    view! { cx,
        <h1 class="title">"Bookings"</h1>
        <ModalAdd is_active=is_active default=Booking::default form=add_form/>
        <div class="buttons">
            <button class="button is-primary" on:click=arse>
                "Add Booking"
            </button>
        </div>
        <BookingList/>
    }
}
