use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use frontend::components;
// use frontend::components::DbConnection;
use frontend::log::wasm_log;
use frontend::store;

use ybc::NavbarFixed::Top;
use ybc::NavbarItemTag::A;

// type Badger = Rc<DbConnection>;
// type Badger = Rc<RefCell<DbConnection>>;
use yew::functional::{use_mut_ref, use_state};
use yew::platform::pinned::oneshot;

#[function_component(App)]
fn app() -> Html {
    wasm_log("refresh_app");
    // let db_connection = use_state(|| components::DbConnection::new());
    // let db_connection: Badger = Rc::new(components::DbConnection::new());

    // let db_connection = use_mut_ref(|| components::DbConnection::new());

    // let (my_store, dispatch) = use_store::<MyStore>();
    // let foo = my_store.conn.clone();

    let (tx, rx) = oneshot::channel::<()>();
    spawn_local(async {
        // todo remove unwrap
        store::connect().await.unwrap();
        wasm_log("connected in main");
        tx.send(()).unwrap();
    });
    rx.into();
    store::DB.wait();

    html! {
      <>
    //   <ContextProvider<Badger> context={db_connection}>

      <ybc::Navbar fixed={Top} navstart={navbar_items()}></ybc::Navbar>
      <ybc::Section>
        <ybc::Container fluid=true>
        <components::UserPage/>
        </ybc::Container>
      </ybc::Section>

    //   </ContextProvider<Badger>>
      </>
    }
}

fn navbar_items() -> Html {
    html! {
        <>
            <ybc::NavbarItem tag={A} href="/">
                { "Home" }
            </ybc::NavbarItem>
            <ybc::NavbarItem tag={A} href="https://github.com/anlumo/tttod/blob/main/README.md">
                { "About" }
            </ybc::NavbarItem>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;
use yewdux::prelude::*;

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Debug, Clone, Store)]
pub struct MyStore {
    pub conn: Rc<Surreal<Client>>,
}

impl Default for MyStore {
    fn default() -> Self {
        Self {
            conn: Rc::new(Surreal::init()),
        }
    }
}

impl PartialEq for MyStore {
    fn eq(&self, _: &Self) -> bool {
        return true;
    }
}
