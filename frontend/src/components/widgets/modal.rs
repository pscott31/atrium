use yew::prelude::*;

// use crate::log::log;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub is_active: bool,
    pub on_close: Callback<()>,
    pub title: String,
    pub footer: Html,
    pub body: Html,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_close = props.on_close.clone();
    let classes = classes!("modal", props.is_active.then(|| "is-active"));
    let close_clicked = move |_| on_close.emit(());

    html! {
        <div class={classes}>
        <div class="modal-background" onclick={close_clicked.clone()}></div>
        <div class="modal-card">
            <header class="modal-card-head">
                <p class="modal-card-title">{props.title.clone()}</p>
                <button class="delete" aria-label="close" onclick={close_clicked.clone()}></button>
            </header>
            <section class="modal-card-body">
                {props.body.clone()}
            </section>
            <footer class="modal-card-foot">
                {props.footer.clone()}
            </footer>
        </div>
        <button class="modal-close is-large" aria-label="close" onclick={close_clicked.clone()}></button>
    </div>}
}
