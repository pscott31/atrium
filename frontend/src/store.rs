use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

///////////////Logging
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

////////// END LOGGING

const USER: &str = "user";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: String,
}
static DB: Surreal<Client> = Surreal::init();

pub async fn get_users() -> surrealdb::Result<Vec<User>> {
    println!("connecting");
    DB.connect::<Ws>("localhost:8000").await?;

    // let db = Surreal::new::<Ws>("localhost:8000").await?;
    // console_log!("signing in");
    // println!("signing in");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // println!("signed in");
    // console_log!("connected");
    // panic!("arse");
    DB.use_ns("atrium").use_db("atrium").await?;

    // let mut accounts = Vec::new();
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
