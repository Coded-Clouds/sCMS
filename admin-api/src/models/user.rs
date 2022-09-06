use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub fullname: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewUser {
    pub fullname: String,
}

