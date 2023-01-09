use frontend::{GreeterClient, HelloRequest};
use tonic;
use tonic_web_wasm_client::Client;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let message = use_state_eq(|| "waiting".to_string());
    get_message(message.clone());
    let message2 = (*message).clone();

    html! {
        <div>
        <h1>{ "Hello World" }</h1>
        <h1>{ message2 }</h1>
        </div>
    }
}

fn get_message(message: UseStateHandle<String>) {
    spawn_local(async move {
        let base_url = String::from("http://localhost:50051"); // URL of the gRPC-web server
        let mut client = GreeterClient::new(Client::new(base_url));
        // let request = tonic::Request::new(HelloRequest {
        //     name: "Dave".into(),
        // });
        let request = HelloRequest {
            name: "Dave".into(),
        };
        let response = client.say_hello(request).await; // Execute your queries the same way as you do with defaule transport layer

        message.set(response.unwrap().get_ref().message.clone());
    })
}

fn main() {
    yew::Renderer::<App>::new().render();
}
