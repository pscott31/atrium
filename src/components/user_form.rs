use crate::store::User;
use leptos::*;

#[component]
pub fn UserForm(cx: Scope, thing: RwSignal<User>) -> impl IntoView {
    let (name, set_name) = create_slice(cx, thing, |u| u.name.clone(), |u, v| u.name = v);
    let (cn, set_cn) = create_slice(
        cx,
        thing,
        |u| u.contact_name.clone(),
        |u, v| u.contact_name = v,
    );
    let (email, set_email) = create_slice(cx, thing, |u| u.email.clone(), |u, v| u.email = v);
    let (phone, set_phone) = create_slice(cx, thing, |u| u.phone.clone(), |u, v| u.phone = v);

    view! { cx,
        <div class="field">
            <label class="label">"Name"</label>
            <div class="control">
                <input
                    prop:value=name
                    class="input"
                    type="text"
                    placeholder="Dave"
                    on:input=move |ev| { set_name(event_target_value(&ev)) }
                />
            </div>
        </div>
        <div class="field">
            <label class="label">"Contact Name"</label>
            <div class="control">
                <input
                    prop:value=cn
                    class="input"
                    type="text"
                    placeholder="Joe Bloggs"
                    on:input=move |ev| { set_cn(event_target_value(&ev)) }
                />
            </div>
        </div>
        <div class="field">
            <label class="label">"Email Address"</label>
            <div class="control">
                <input
                    prop:value=email
                    class="input"
                    type="text"
                    placeholder="joe@bloggs.com"
                    on:input=move |ev| { set_email(event_target_value(&ev)) }
                />
            </div>
        </div>
        <div class="field">
            <label class="label">"Phone Number"</label>
            <div class="control">
                <input
                    prop:value=phone
                    class="input"
                    type="text"
                    placeholder="01234 444 232"
                    on:input=move |ev| { set_phone(event_target_value(&ev)) }
                />
            </div>
        </div>
    }
}
