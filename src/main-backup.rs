use leptos::*;
mod log;
mod store;

extern crate console_error_panic_hook;
use std::panic;

pub async fn dave() -> Result<(), String> {
    return Ok(());
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    //let initial_users = store::get_bookings();
    log!("hi");

    let (connected, set_connected) = create_signal(cx, false);

    let connecty_resource = create_resource(
        cx,
        || (),
        move |_| async move {
            let res = store::connect().await;
            match res {
                Ok(_) => {
                    log!("setting connected!");
                    set_connected(true);
                    "connected".to_string()
                }
                Err(err) => format!("oops {}", err),
            }
        }, //todo unwrappy
    );

    let connect_result = move || {
        connecty_resource
            .read(cx)
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    let user_resource = create_resource(
        cx,
        connected,
        move |_| async move {
            log!("user resource connected? {}", connected());
            match connected() {
                false => Vec::<store::User>::new(),
                true => store::get_users().await.unwrap(),
            }
        }, //todo unwrappy
    );

    let dudgers = move || match user_resource.read(cx) {
        Some(value) => value,
        None => Vec::<store::User>::new(),
    };

    view! { cx,
          <h1>"User List"</h1>
        <div>{connect_result}</div>
        <div>"bobo"</div>
    <For
        each = dudgers
        key=|u| u.name.clone()
        view=move |cx, cake| { view! {cx, <li> {cake.name} </li>}  }

    />


    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(|cx| view! { cx,  <App/> })
}
