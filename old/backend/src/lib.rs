pub use protos::greeter_client::GreeterClient;
pub use protos::HelloRequest;

pub mod protos {
    tonic::include_proto!("atrium");
}
