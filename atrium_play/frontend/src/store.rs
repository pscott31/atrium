use std::time::{self, SystemTime};

use crate::log::log;
use anyhow::Result;
use async_trait::async_trait;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

const USER: &str = "user";
const BOOKING: &str = "booking";

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Default)]
#[allow(dead_code)]
pub struct User {
    #[serde(skip_serializing)]
    id: Option<String>,
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: String,
}

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
    Ok(accounts)
}

#[async_trait]
pub trait Addable: Sized {
    async fn add(&self) -> Result<Self, String>;
}

#[async_trait]
impl Addable for User {
    async fn add(&self) -> Result<User, String> {
        log!("adding user {self:?}");
        let db = DB.get().expect("db not connected");
        let res: Result<Self, _> = db.create(USER).content(self).await;

        match res {
            Ok(stuff @ User { .. }) => {
                log!("OK: {stuff:?}");
                Ok(stuff)
            }
            Err(err) => {
                log!("OH NO: {err:?}");
                Err("fucksticks".into()) //todo bettor error!
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[allow(dead_code)]
pub struct Booking {
    #[serde(skip_serializing)]
    id: Option<String>,
    pub user: String,
    pub start_time: time::SystemTime,
    pub duration: time::Duration,
}

impl Default for Booking {
    fn default() -> Self {
        Self {
            id: Default::default(),
            user: Default::default(),
            start_time: time::SystemTime::now(),
            duration: Default::default(),
        }
    }
}

pub async fn get_bookings() -> surrealdb::Result<Vec<Booking>> {
    log!("get_users()");
    let db = DB.get().expect("db not connected");

    let mut bookings: Vec<Booking> = db.select(BOOKING).await?;

    let u3 = Booking {
        id: None,
        user: "".into(),
        start_time: SystemTime::now(),
        duration: time::Duration::from_secs(60 * 60),
    };
    bookings.push(u3);
    println!("{bookings:?}");

    Ok(bookings)
}

#[async_trait]
impl Addable for Booking {
    async fn add(&self) -> Result<Booking, String> {
        log!("adding booking {self:?}");
        let db = DB.get().expect("db not connected");
        let res: Result<Self, _> = db.create(BOOKING).content(self).await;

        match res {
            Ok(stuff @ Booking { .. }) => {
                log!("OK: {stuff:?}");
                Ok(stuff)
            }
            Err(err) => {
                log!("OH NO: {err:?}");
                Err("fucksticks".into()) //todo bettor error!
            }
        }
    }
}
