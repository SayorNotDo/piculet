use axum::Json;
use serde::{Deserialize, Serialize};
use crate::dao::user::{User, UserDao};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub access_token: String,
}


pub async fn login(request: Json<Request>) {
    let user = User::new(&request.username, &request.password, false);
    let user_dao = UserDao::new();
    let response = match User {  };
}