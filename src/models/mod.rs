use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: Option<Thing>,
    name: String,
    email: String,
}
