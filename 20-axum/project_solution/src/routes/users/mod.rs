use serde::{Deserialize, Serialize};

pub mod create_user;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequesCreatetUser {
    username: String,
    password: String,
}