use serde::Deserialize;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

const USER: &str = "user";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct User {
    name: String,
    contact_name: String,
    email: String,
    phone: String,
}

pub async fn get_users() -> surrealdb::Result<Vec<User>> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("atrium").use_db("atrium").await?;

    let accounts: Vec<User> = db.select(USER).range("one".."two").await?;

    println!("{accounts:?}");

    Ok(accounts)
}
