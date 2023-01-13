use crate::log::log;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
////////// END LOGGING

const USER: &str = "user";

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[allow(dead_code)]
pub struct User {
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: String,
}

// static DB: Surreal<Client> = Surreal::init();
//static Potato: std::cell::RefCell<Surreal<Client>> = Surreal::init();
// static Potato: std::cell::RefCell<String>; // = std:cell::RefCell::new("foo".into());

pub static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub async fn connect() -> surrealdb::Result<()> {
    log!("connecting properly");
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    // let bandit = Surreal<Ws>::init();
    //db.connect::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // TODO - remove panic?
    DB.set(db).expect("database already connected");
    // std::mem::swap(&mut db, &mut DB);
    Ok(())
}

pub async fn get_users() -> surrealdb::Result<Vec<User>> {
    log!("connecting");
    // TODO: Don't panic
    let db = DB.get().expect("db not connected");
    db.connect::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("atrium").use_db("atrium").await?;

    let mut accounts: Vec<User> = db.select(USER).await?;

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

pub async fn add_user() {}
