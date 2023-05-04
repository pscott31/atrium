use std::time::{self, SystemTime};

use crate::log::log;
// use anyhow::Ok;
use async_trait::async_trait;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

const USER: &str = "user";
const BOOKING: &str = "booking";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Error {
    ConnectionError(String),
    SigninError(String),
    NsError(String),
    UpdateFailed(String),
    InsertFailed(String),
    SelectFailed(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::ConnectionError(reason) => write!(f, "db connect failed: {}", reason),
            Error::SigninError(reason) => write!(f, "db signin failed: {}", reason),
            Error::NsError(reason) => write!(f, "db use namespace failed: {}", reason),
            Error::UpdateFailed(reason) => write!(f, "update failed: {}", reason),
            Error::InsertFailed(reason) => write!(f, "insert failed: {}", reason),
            Error::SelectFailed(reason) => write!(f, "insert failed: {}", reason),
        }
    }
}

#[async_trait]
pub trait Addable: Sized {
    async fn add(&self) -> Result<Self, Error>;
}

#[async_trait]
pub trait Updatable: Sized {
    async fn update(&self) -> Result<(), Error>;
}

pub static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub fn get_db() -> Result<&'static Surreal<Client>, Error> {
    let db = DB
        .get()
        .ok_or_else(|| Error::ConnectionError("db not connected".into()))?;
    Ok(db)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Default)]
#[allow(dead_code)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: String,
}

pub async fn connect() -> Result<(), Error> {
    log!("connecting..");
    let db = Surreal::new::<Http>("localhost:8000")
        .await
        .map_err(|e| Error::ConnectionError(e.to_string()))?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| Error::SigninError(e.to_string()))?;

    db.use_ns("atrium")
        .use_db("atrium")
        .await
        .map_err(|e| Error::NsError(e.to_string()))?;
    DB.set(db).expect("database already connected");

    // make_dummy_data().await.unwrap();
    Ok(())
}

pub async fn get_users() -> Result<Vec<User>, Error> {
    let db = get_db()?;
    db.select(USER)
        .await
        .map_err(|e| Error::SelectFailed(e.to_string()))
}

#[async_trait]
impl Addable for User {
    async fn add(&self) -> Result<User, Error> {
        let db = get_db()?;
        let res: Result<Self, _> = db.create(USER).content(self).await;

        match res {
            Ok(u) => Ok(u),
            Err(err) => Err(Error::UpdateFailed(err.to_string())),
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

pub async fn get_bookings() -> Result<Vec<Booking>, Error> {
    let db = get_db()?;
    let mut bookings: Vec<Booking> = db
        .select(BOOKING)
        .await
        .map_err(|e| Error::SelectFailed(e.to_string()))?;

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
    async fn add(&self) -> Result<Booking, Error> {
        log!("adding booking {self:?}");
        let db = DB
            .get()
            .ok_or_else(|| Error::ConnectionError("db not connected".into()))?;
        let res: Result<Self, _> = db.create(BOOKING).content(self).await;

        match res {
            Ok(b) => Ok(b),
            Err(err) => Err(Error::InsertFailed(err.to_string())),
        }
    }
}

#[async_trait]
impl Updatable for User {
    async fn update(&self) -> Result<(), Error> {
        let db = get_db()?;
        let id = self
            .id
            .clone()
            .ok_or_else(|| Error::UpdateFailed("id required".into()))?;

        db.update(id)
            .content(self)
            .await
            .map_err(|e| Error::UpdateFailed(e.to_string()))
            .map(|_: User| ())
    }
}
