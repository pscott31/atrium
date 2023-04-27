use crate::components::widgets::Modal;
use crate::components::UserForm;
// use crate::state::GlobalState;
use crate::store;
use crate::store::User;
use yew::prelude::*;
// use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddUserModalProps {
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub visible: bool,
}

pub struct AddModal<T> {
    thing: T,
    error: Option<String>,
}

pub enum Msg<T> {
    CloseRequest,
    Updated(T),
    SaveRequest,
    Saved(T),
    SaveError(String),
}

impl<T: Default + Clone + store::Addable + HasForm + 'static> Component for AddModal<T> {
    type Message = Msg<T>;
    type Properties = AddUserModalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        AddModal {
            thing: T::default(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CloseRequest => {
                ctx.props().on_close.emit(());
                false
            }
            Msg::SaveRequest => {
                let link = ctx.link().clone();
                let user = self.thing.clone();
                link.send_future(async move {
                    match user.add().await {
                        Ok(_) => Msg::Saved(user),
                        Err(err) => Msg::SaveError(format!("{err}")),
                    }
                });
                false
            }
            Msg::Saved(user) => {
                // TODO
                // Dispatch::<GlobalState>::new().reduce_mut(|s| s.users.push(user));
                ctx.link().send_message(Msg::CloseRequest);
                false
            }
            Msg::SaveError(err) => {
                self.error = Some(err);
                true
            }
            Msg::Updated(u) => {
                self.thing = u;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let l = ctx.link();
        let footer = html! {<>
            <button class="button is-success" onclick={l.callback(|_| Msg::SaveRequest)}>{"Save changes"}</button>
            <button class="button" onclick={l.callback(|_| Msg::CloseRequest)}>{"Cancel"}</button>
            </>
        };

        html! {
            <Modal title="Add User" {footer} visible={ctx.props().visible} on_close={l.callback(|_| Msg::CloseRequest)}>
                if let Some(err) = &self.error {
                    <div class="notification is-danger">{format!("Error adding user: {err}")}</div>
                }
                {
                    T::form(ctx)
                }

            </Modal>
        }
    }
}

pub trait HasForm: Sized {
    fn form<C>(ctx: &Context<C>) -> Html
    where
        C: Component<Message = Msg<Self>>;
}

impl HasForm for User {
    fn form<C>(ctx: &Context<C>) -> Html
    where
        C: Component<Message = Msg<User>>,
    {
        html! {
            <UserForm on_update={ctx.link().callback(|u| Msg::Updated(u))} />
        }
    }
}
