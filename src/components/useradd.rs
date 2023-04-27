use leptos::*;


#[component]
pub fn UserAdd(cx: Scope, on_save: ReadSignal<()>) -> impl IntoView {

    create_effect(on_save, )

    view! { cx,
        <div class="field">
            <label class="label">"Name"</label>
            <div class="control">
                <input class="input" type="text" placeholder="Dave"/>
            </div>
        </div>
        <div class="field">
            <label class="label">"Contact Name"</label>
            <div class="control">
                <input class="input" type="text" placeholder="Joe Bloggs"/>
            </div>
        </div>
        <div class="field">
            <label class="label">"Email Address"</label>
            <div class="control">
                <input class="input" type="text" placeholder="joe@bloggs.com"/>
            </div>
        </div>
        <div class="field">
            <label class="label">"Phone Number"</label>
            <div class="control">
                <input class="input" type="text" placeholder="01234 444 232"/>
            </div>
        </div>
    }
}
