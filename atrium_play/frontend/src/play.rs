use frontend::store;
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let users = store::get_users().await.unwrap();
    println!("{:?}", users);
}
