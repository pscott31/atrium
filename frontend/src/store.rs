use crate::log::log;
use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
const USER: &str = "user";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[allow(dead_code)]
pub struct User {
    #[serde(skip_serializing)]
    id: Option<String>,
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: String,
}

// static DB: Surreal<Client> = Surreal::init();
//static Potato: std::cell::RefCell<Surreal<Client>> = Surreal::init();
// static Potato: std::cell::RefCell<String>; // = std:cell::RefCell::new("foo".into());

pub static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub async fn connect() -> Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("atrium").use_db("atrium").await?;
    DB.set(db).expect("database already connected");
    Ok(())
}

pub async fn get_users() -> surrealdb::Result<Vec<User>> {
    log!("get_users()");
    // TODO: Don't panic
    let db = DB.get().expect("db not connected");

    let mut accounts: Vec<User> = db.select(USER).await?;

    let u3 = User {
        id: None,
        name: "xxx".into(),
        contact_name: "yyy".into(),
        email: "emaily".into(),
        phone: "phone".into(),
    };
    accounts.push(u3);
    println!("{accounts:?}");

    Ok(accounts)
}

pub async fn add_user(user: &User) -> Result<(), String> {
    log!("adding user {user:?}");
    let db = DB.get().expect("db not connected");
    let res: Result<User, _> = db.create(USER).content(user).await;

    match res {
        Ok(stuff @ User { .. }) => {
            log!("OK: {stuff:?}");
            Ok(())
        }
        Err(err) => {
            log!("OH NO: {err:?}");
            Err("fucksticks".into()) //todo bettor error!
        }
    }
}
