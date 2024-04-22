use axum::{extract::Extension, response::Json};
use serde::{Deserialize};
use crate::config::Config;
use crate::errors::CustomError;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub access_token: String,
}


pub async fn login(Extension(pool): Extension<db::Pool>,
                   Extension(config): Extension<Config>,
                   request: Json<Request>,
) -> Result<Json<Response>, CustomError> {
    // 登录逻辑
    let client = pool.get().await?;

    let user = User::new(&request.username, &request.password, false);

    let response = match {};

    Ok(Json())
}