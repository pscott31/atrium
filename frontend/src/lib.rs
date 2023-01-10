pub mod components;
pub mod proto;

pub mod store;
pub use proto::greeter_client::GreeterClient;
pub use proto::HelloRequest;

// pub mod protos {
//     tonic::include_proto!("atrium");
// }
