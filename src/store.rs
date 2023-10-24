use crate::log::log;
use async_trait::async_trait;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

//////////////////////////////////////////////////////////////////// Errors

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Error {
    Connection(String),
    Signin(String),
    Ns(String),
    Update(String),
    Insert(String),
    Select(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Connection(reason) => write!(f, "db connect failed: {}", reason),
            Error::Signin(reason) => write!(f, "db signin failed: {}", reason),
            Error::Ns(reason) => write!(f, "db use namespace failed: {}", reason),
            Error::Update(reason) => write!(f, "update failed: {}", reason),
            Error::Insert(reason) => write!(f, "insert failed: {}", reason),
            Error::Select(reason) => write!(f, "insert failed: {}", reason),
        }
    }
}

//////////////////////////////////////////////////////////////////// Traits

pub trait Entity:
    Sized + Send + Sync + Debug + Clone + Serialize + for<'de> Deserialize<'de>
{
    const TABLE: &'static str;
    fn get_id(&self) -> Option<Thing>;
}

#[async_trait]
pub trait Addable: Entity {
    async fn add(&self) -> Result<Self, Error> {
        log!("adding {} {self:?}", Self::TABLE);
        let db = get_db()?;
        let res: Result<Self, _> = db.create(Self::TABLE).content(self).await;

        match res {
            Ok(b) => Ok(b),
            Err(err) => Err(Error::Insert(err.to_string())),
        }
    }
}

#[async_trait]
pub trait Updatable: Entity
where
    Self:,
{
    async fn update(&self) -> Result<(), Error> {
        let db = get_db()?;
        let id = self
            .get_id()
            .ok_or_else(|| Error::Update("id required".into()))?;

        db.update(id)
            .content(self)
            .await
            .map_err(|e| Error::Update(e.to_string()))
            .map(|_: Self| ())
    }
}

//////////////////////////////////////////////////////////////////// Database

pub static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub fn get_db() -> Result<&'static Surreal<Client>, Error> {
    let db = DB
        .get()
        .ok_or_else(|| Error::Connection("db not connected".into()))?;
    Ok(db)
}

//////////////////////////////////////////////////////////////////// Users

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
        .map_err(|e| Error::Connection(e.to_string()))?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| Error::Signin(e.to_string()))?;

    db.use_ns("atrium")
        .use_db("atrium")
        .await
        .map_err(|e| Error::Ns(e.to_string()))?;
    DB.set(db).expect("database already connected");

    Ok(())
}

pub async fn get_users() -> Result<Vec<User>, Error> {
    let db = get_db()?;
    db.select(User::TABLE)
        .await
        .map_err(|e| Error::Select(e.to_string()))
}

#[async_trait]
impl Updatable for User {}
impl Addable for User {}
impl Entity for User {
    const TABLE: &'static str = "user";
    fn get_id(&self) -> Option<Thing> {
        self.id.clone()
    }
}

//////////////////////////////////////////////////////////////////// Bookings

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Booking {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub user: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub duration: chrono::Duration,
}

impl Default for Booking {
    fn default() -> Self {
        Self {
            id: Default::default(),
            user: Default::default(),
            start_time: chrono::Utc::now(),
            duration: chrono::Duration::seconds(0),
        }
    }
}

pub async fn get_bookings() -> Result<Vec<Booking>, Error> {
    let db = get_db()?;
    let bookings: Vec<Booking> = db
        .select(Booking::TABLE)
        .await
        .map_err(|e| Error::Select(e.to_string()))?;

    // let u3 = Booking {
    //     id: None,
    //     user: "fred".into(),
    //     start_time: chrono::Utc::now(),
    //     duration: chrono::Duration::seconds(30 * 60),
    // };
    // bookings.push(u3);
    // println!("{bookings:?}");

    Ok(bookings)
}

#[async_trait]
impl Addable for Booking {}

#[async_trait]
impl Updatable for Booking {}

impl Entity for Booking {
    const TABLE: &'static str = "booking";
    fn get_id(&self) -> Option<Thing> {
        self.id.clone()
    }
}
