use axum::Extension;
use chrono::{DateTime, Utc};
use crypto_utils::sha::{Algorithm, CryptographicHash};
use serde::Serialize;
use tokio_postgres::error::DbError;
use uuid::Uuid;

use crate::dao::base::BaseDao;
use crate::errors::CustomError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: &str, password: &str, gen_uuid: bool) -> Self {
        let username = username.to_lowercase();

        // salting the password
        let password = format!("{username}${password}");

        // hash the password using SHA-512 algorithm and encode it into String.
        let hashed_password = hex::encode(CryptographicHash::hash(
            Algorithm::SHA512,
            password.as_bytes(),
        ));

        // generate UUID
        let uuid = if gen_uuid {
            Uuid::new_v4()
        } else {
            Uuid::nil()
        };

        Self {
            id: 0,
            uuid,
            username,
            hashed_password,
            email: "".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub struct UserDao {
    pool: db::Pool,
}

impl UserDao {
    fn new(Extension(pool): Extension<db::Pool>) -> Self {
        UserDao { pool }
    }
}

impl BaseDao<User> for UserDao {
    async fn insert(&self, object: &User) -> Result<(), CustomError> {
        // 实现插入用户到数据库的逻辑
        let client = self.pool.get().await?;

        let _ = db::queries::users::insert_user()
            .bind(&client, &object.uuid, &object.username, &object.hashed_password)
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, _id: i32) -> Result<User, DbError> {
        todo!()
    }

    async fn update(&self, _object: &User) -> Result<User, DbError> {
        todo!()
    }

    async fn delete_by_id(&self, _id: i32) -> Result<User, DbError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check if the username is in lowercase.
    #[test]
    fn test_username_is_lowercase() {
        let username = "MeDZiK";
        let password = "password";

        let username_expected = "medzik";

        let user = User::new(username, password, false);
        assert_eq!(user.username, username_expected)
    }

    // Check if the password is hashed.
    #[test]
    fn test_password_hashed() {
        let username = "username";
        let password = "password";

        let user = User::new(username, password, false);


        assert_ne!(user.hashed_password, password)
    }

    #[tokio::test]
    async fn test_insert() {
        let username = "username";
        let password = "password";
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = db::create_pool(&db_url);
        let user = User::new(username, password, true);

        let user_dao = UserDao::new(Extension(pool));
        let result = user_dao.insert(&user).await;

        assert!(result.is_ok());
    }
}