use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
pub struct DbConnection {
    pub conn: Surreal<Client>,
}

enum DbOp {
    Connect,
    ListUsers,
}

// impl DbConnection {
//     pub fn new() -> Self {
//         Self {
//             conn: Surreal::init(),
//         }
//     }
// }

// impl PartialEq for DbConnection {
//     fn eq(&self, _: &Self) -> bool {
//         true
//     }
// }
