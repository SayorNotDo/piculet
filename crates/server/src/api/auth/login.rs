use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub access_token: String,
}


pub async fn login() {}