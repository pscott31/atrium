use crate::components::input_field;
use crate::store::User;
use yew::prelude::*;
use yew_icons::IconId;

pub enum Msg {
    NameChanged(String),
    ContactNameChanged(String),
    EmailChanged(String),
    PhoneChanged(String),
}

#[derive(Properties, PartialEq)]
pub struct UserFormProps {
    pub on_update: Callback<User>,
}

pub struct UserForm {
    user: User,
}

impl Component for UserForm {
    type Message = Msg;
    type Properties = UserFormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user: User::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NameChanged(s) => self.user.name = s,
            Msg::ContactNameChanged(s) => self.user.contact_name = s,
            Msg::EmailChanged(s) => self.user.email = s,
            Msg::PhoneChanged(s) => self.user.phone = s,
        }
        ctx.props().on_update.emit(self.user.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            {input_field(ctx, "name".into(), "Display Name".into(), self.user.name.clone(), |s| Msg::NameChanged(s), IconId::FontAwesomeRegularUser)}
            {input_field(ctx, "contact_name".into(), "Contact Name".into(), self.user.contact_name.clone(), |s| Msg::ContactNameChanged(s), IconId::FontAwesomeRegularUser)}
            {input_field(ctx, "email".into(), "Contact Email Address".into(), self.user.email.clone(), |s| Msg::EmailChanged(s), IconId::FontAwesomeRegularEnvelope)}
            {input_field(ctx, "phone".into(), "Contact Telephone Number".into(), self.user.phone.clone(), |s| Msg::PhoneChanged(s), IconId::FontAwesomeSolidPhone)}

            <div>{format!("name: {}", self.user.name)}</div>
            <div>{format!("contact_name: {}", self.user.contact_name)}</div>
            <div>{format!("email: {}", self.user.email)}</div>
            <div>{format!("phone: {}", self.user.phone)}</div>
            </>
        }
    }
}
