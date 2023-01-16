use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    is_active: bool,
    on_close: Callback<()>,
    title: Html,
    footer: Html,
    body: Html,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    // let is_active = use_state(|| false);

    // {
    //     is_active = is_active.clone();
    //     let close_cb = Callback::from(move |_| is_active.set(false));
    // }

    let on_close = props.on_close.clone();
    let classes = classes!("modal", props.is_active.then(|| "is_active"));
    let close_clicked = move |_| on_close.emit(());

    html! {
        <div class={classes}>
        <div class="modal-background" onclick={close_clicked.clone()}></div>
        <div class="modal-card">
            <header class="modal-card-head">
                // <p class="modal-card-title">{props.title.clone()}</p>
                <button class="delete" aria-label="close" onclick={close_clicked.clone()}></button>
            </header>
            <section class="modal-card-body">
                // {props.body.clone()}
            </section>
            <footer class="modal-card-foot">
                // {props.body.clone()}
            </footer>
        </div>
        <button class="modal-close is-large" aria-label="close" onclick={close_clicked.clone()}></button>
    </div>}
}
