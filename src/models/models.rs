use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = users)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
