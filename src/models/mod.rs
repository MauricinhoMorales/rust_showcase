use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: Option<Thing>,
    name: String,
    email: String,
}
