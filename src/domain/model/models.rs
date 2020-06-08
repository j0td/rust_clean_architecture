use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
}
