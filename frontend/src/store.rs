use crate::log::log;
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

////////// END LOGGING

const USER: &str = "user";

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[allow(dead_code)]
pub struct User {
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: String,
}

static DB: Surreal<Client> = Surreal::init();

pub async fn get_users() -> surrealdb::Result<Vec<User>> {
    log!("connecting");
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("atrium").use_db("atrium").await?;

    let mut accounts: Vec<User> = DB.select(USER).await?;

    let u3 = User {
        name: "xxx".into(),
        contact_name: "yyy".into(),
        email: "emaily".into(),
        phone: "phone".into(),
    };
    accounts.push(u3);
    println!("{accounts:?}");

    Ok(accounts)
}
