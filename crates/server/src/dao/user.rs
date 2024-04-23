use axum::Extension;
use crypto_utils::sha::{Algorithm, CryptographicHash};
use serde::Serialize;
use uuid::Uuid;
use crate::errors::CustomError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: NavieDatetime,
    pub updated_at: NavieDatetime,
}

impl User {
    pub fn new(username: &str, password: &str, gen_uuid: bool) -> Self {
        let username = username.to_lowercase();

        // salting the password
        let password = format!("{username}${password}");

        // hash the password using SHA-512 algorithm and encode it into String.
        let password = hex::encode(CryptographicHash::hash(
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
            uuid,
            username,
            hashed_password,
        }
    }
}

pub struct UserDao {
    pool: db::Pool
}

impl UserDao {
    fn new(Extension(pool): Extension<db::Pool>) -> Self {
        UserDao{ pool }
    }

    async fn create_user(&self, user: User) -> Result<(), Err()> {

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
    async fn test_create_user() {
        let username = "username";
        let password = "password";

        let db_url = std::env::var("DATABASE_URL").unwrap();

        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();

        let user = User::new(username, password, true);
        let result = db::queries::users::insert_user()
        .bind(&client, &user.uuid, &user.username, &user.hashed_password).await.unwrap();

        assert_eq!(1, 1)
    }
}