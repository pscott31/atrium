use crate::store::Booking;
use chrono::{Local, TimeZone, Utc};
use leptos::*;
use chrono_humanize::{Accuracy, HumanTime, Tense};

#[component]
pub fn BookingForm(cx: Scope, thing: RwSignal<Booking>) -> impl IntoView {
    let id = move || {
        thing()
            .id
            // .clone()
            .map(|t| t.to_string())
            .unwrap_or("".to_string())
    };

    let (user, set_user) = create_slice(cx, thing, |u| u.user.clone(), |b, v| b.user = v);
    let (start, set_start) = create_slice(
        cx,
        thing,
        |b| b.start_time.with_timezone(&Local).format("%Y-%m-%dT%H:%M").to_string(),
        |b, v| {
            let res = Local.datetime_from_str(&v, "%Y-%m-%dT%H:%M");
            // TODO - show fails somehow
            if let Ok(t) = res {
                b.start_time = t.with_timezone(&Utc);
            }
        },
    );

    let (end, set_end) = create_slice(
        cx,
        thing,
        |b| (b.start_time + b.duration).with_timezone(&Local).format("%Y-%m-%dT%H:%M").to_string(),
        |b, v| {
            let res = Local.datetime_from_str(&v, "%Y-%m-%dT%H:%M");
            // TODO - show fails somehow
            if let Ok(t) = res {
                b.duration = t.with_timezone(&Utc) - b.start_time
            }
        },
    );

    let duration = move|| HumanTime::from(thing().duration).to_text_en(Accuracy::Precise, Tense::Present);
    view! { cx,
        <div class="field">
            <label class="label">"User"</label>
            <div class="control">
                <input
                    prop:value=user
                    class="input"
                    type="text"
                    placeholder="Dave"
                    on:input=move |ev| { set_user(event_target_value(&ev)) }
                />
            </div>
        </div>
        <div class="field">
            <label class="label">"Start Time"</label>
            <div class="control">
                <input
                    prop:value=start
                    class="input"
                    type="datetime-local"
                    step=15 * 60
                    on:input=move |ev| { set_start(event_target_value(&ev)) }
                />
            </div>
        </div>
        <div class="field">
            <label class="label">"End Time"</label>
            <div class="control">
                <input
                    prop:value=end
                    min=start
                    class="input"
                    type="datetime-local"
                    step=15 * 60
                    on:input=move |ev| { set_end(event_target_value(&ev)) }
                />
            </div>
        </div>
        <div class="field">
            <label class="label">"Duration"</label>
            <div class="control">
                <input
                    prop:value=duration
                    min=start
                    disabled=true
                    readonly=true
                    class="input"
                    type="text"
                />
            </div>
        </div>
        <div class="field">
            <label class="label">"Id"</label>
            <div class="control">
                <input prop:value=id class="input" type="text" readonly=true disabled=true/>
            </div>
        </div>
    }
}
