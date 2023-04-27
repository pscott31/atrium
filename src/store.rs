use std::time::{self, SystemTime};

use crate::log::log;
use async_trait::async_trait;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{thing, Thing};
use surrealdb::Surreal;

const USER: &str = "user";
const BOOKING: &str = "booking";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum StoreError {
    ConnectionError(String),
    SigninError(String),
    NsError(String),
}

impl core::fmt::Display for StoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StoreError::ConnectionError(reason) => write!(f, "db connect failed: {}", reason),
            StoreError::SigninError(reason) => write!(f, "db signin failed: {}", reason),
            StoreError::NsError(reason) => write!(f, "db use namespace failed: {}", reason),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[allow(dead_code)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: String,
}

pub async fn make_dummy_data() -> anyhow::Result<()> {
    let fred = User {
        id: Some(thing("user:fred123").unwrap()),
        contact_name: "fredrick".to_string(),
        name: "fred".to_string(),
        email: "fred@bloggs.com".to_string(),
        phone: "1234".to_string(),
    };

    let db = DB.get().expect("db not connected");
    log!("creating");
    let u: User = db.create(USER).content(fred).await?;
    log!("{:?}", u);
    Ok(())
}

pub static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub async fn connect() -> Result<(), StoreError> {
    log!("connecting..");
    let db = Surreal::new::<Http>("localhost:8000")
        .await
        .map_err(|e| StoreError::ConnectionError(e.to_string()))?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| StoreError::SigninError(e.to_string()))?;

    db.use_ns("atrium")
        .use_db("atrium")
        .await
        .map_err(|e| StoreError::NsError(e.to_string()))?;
    DB.set(db).expect("database already connected");

    // make_dummy_data().await.unwrap();
    Ok(())
}

pub async fn get_users() -> surrealdb::Result<Vec<User>> {
    log!("get_users()");
    let db = DB.get().expect("db not connected");
    let accounts: Vec<User> = db.select(USER).await?;
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
