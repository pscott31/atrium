use super::{BookingForm, BookingFormProps, ModalEdit, ModalEditProps};
use crate::store::{get_bookings, Booking};
use chrono::Local;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use leptos::*;

#[component]
pub fn BookingList(cx: Scope) -> impl IntoView {
    let booking_res = create_resource(
        cx,
        || (),
        move |_| async move { get_bookings().await.unwrap() },
    );

    let bookings = move || match booking_res.read(cx) {
        Some(value) => value,
        None => Vec::<Booking>::new(),
    };

    let is_active = create_rw_signal(cx, false);
    let edit_form = |cx: Scope, booking| view! { cx, <BookingForm thing=booking/> };
    let editing_booking = create_rw_signal(cx, Booking::default());

    let edit_clicked = move |b: RwSignal<Booking>| {
        move |_| {
            editing_booking.set(b());
            is_active.set(true);
        }
    };

    view! { cx,
        <ModalEdit is_active=is_active thing=editing_booking form=edit_form/>
        <table class="table">
            <thead>
                <tr>
                    <th>"Date"</th>
                    <th>"Time"</th>
                    <th>"Duration"</th>
                    <th>"User"</th>
                    <th></th>
                </tr>
            </thead>
            <For
                each=bookings
                key=|key| key.id.clone()
                view=move |cx, user| {
                    let u = create_rw_signal(cx, user);
                    view! { cx,
                        <tr>
                            <td>{u().start_time.with_timezone(&Local).format("%Y-%m-%d").to_string()}</td>
                            <td>{u().start_time.with_timezone(&Local).format("%H:%M").to_string()}</td>
                            <td>{HumanTime::from(u().duration).to_text_en(Accuracy::Precise, Tense::Present)}</td>
                            <td>{u().user}</td>
                            <td>
                                <button class="button is-primary is-small" on:click=edit_clicked(u)>
                                    "Edit"
                                </button>
                            </td>
                        </tr>
                    }
                }
            />
        </table>
    }
}
