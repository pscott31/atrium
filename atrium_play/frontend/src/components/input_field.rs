use yew::prelude::*;
use yew_icons::{Icon, IconId};

pub fn input_field<T: BaseComponent>(
    ctx: &Context<T>,
    name: String,
    label: String,
    value: String,
    msg_fn: fn(String) -> T::Message,
    icon_id: IconId,
) -> Html {
    let update = ctx.link().callback(msg_fn);

    html! {
      <ybc::Field label={label}>
          <ybc::Control classes="has-icons-left">
            <ybc::Input {name} {value} {update}/>
            <ybc::Icon alignment={ybc::Alignment::Left} size={ybc::Size::Small}><Icon {icon_id}/></ybc::Icon>
          </ybc::Control>
        </ybc::Field>
    }
}
