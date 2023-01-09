use frontend::{GreeterClient, HelloRequest};
use tonic_web_wasm_client::Client;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use ybc::NavbarFixed::Top;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let message = use_state_eq(|| "waiting".to_string());
    get_message(message.clone());
    let message2 = (*message).clone();

    html! {
      <>
      <ybc::Navbar fixed={Top} /* .. your navbar content here .. *//>
      <ybc::Container fluid=true>
        <ybc::Tile ctx={Ancestor}>
          <ybc::Tile ctx={Parent} vertical=true size={Four}>
            <ybc::Tile ctx={Child} classes={classes!("box")}>
              <p>{"Lorem ipsum dolor sit amet ..."}</p>
            </ybc::Tile>
            /* .. snip .. more tiles here .. */
          </ybc::Tile>
        </ybc::Tile>
      </ybc::Container>
      </>
    }

    // html! {
    //       <section class="section">
    //   <div class="container">
    //     <h1 class="title">{"Atrium"}</h1>
    //     <h2>          { message2 }</h2>
    //     <p class="subtitle">{"My first website with "}<strong>{"Bulma"}</strong>{"!"}
    //     </p>
    //   </div>
    // </section>

    //   }
}

fn get_message(message: UseStateHandle<String>) {
    spawn_local(async move {
        let grpc_web_client = Client::new("http://localhost:50051".into());
        let mut client = GreeterClient::new(grpc_web_client);
        let request = HelloRequest {
            name: "Dave".into(),
        };
        let response = client.say_hello(request).await;
        message.set(response.unwrap().get_ref().message.clone());
    })
}

fn main() {
    yew::Renderer::<App>::new().render();
}
