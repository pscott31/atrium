use crate::components::widgets::Modal;
use crate::components::UserForm;
use crate::state::GlobalState;
use crate::store;
use crate::store::User;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddUserModalProps {
    on_close: Callback<()>,
}

pub struct AddUserModal<T> {
    user: T,
    error: Option<String>,
}

pub enum Msg {
    CloseRequest,
    Updated(User),
    SaveRequest,
    Saved(User),
    SaveError(String),
}

impl<T> Component for AddUserModal<T> {
    type Message = Msg;
    type Properties = AddUserModalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        AddUserModal {
            user: User::default(),
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
                let user = self.user.clone();
                link.send_future(async move {
                    match store::add_user(&user).await {
                        Ok(_) => Msg::Saved(user),
                        Err(err) => Msg::SaveError(format!("{err}")),
                    }
                });
                false
            }
            Msg::Saved(user) => {
                Dispatch::<GlobalState>::new().reduce_mut(|s| s.users.push(user));
                ctx.link().send_message(Msg::CloseRequest);
                false
            }
            Msg::SaveError(err) => {
                self.error = Some(err);
                true
            }
            Msg::Updated(u) => {
                self.user = u;
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
            <Modal title="Add User" {footer} on_close={l.callback(|_| Msg::CloseRequest)}>
                if let Some(err) = &self.error {
                    <div class="notification is-danger">{format!("Error adding user: {err}")}</div>
                }
                <UserForm on_update={l.callback(|u| Msg::Updated(u))} />
            </Modal>
        }
    }
}
