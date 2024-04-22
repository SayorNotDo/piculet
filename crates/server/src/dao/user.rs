use crypto_utils::sha::{Algorithm, CryptographicHash};
use serde::Serialize;
use uuid::Uuid;
use crate::errors::CustomError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
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
            Uuid::new_v4().to_string()
        } else {
            "none".to_string()
        };

        Self {
            uuid,
            username,
            password,
        }
    }
}

pub struct UserDao {
    pub pool: db::Pool
}

impl UserDao {
    pub async fn create_user(&self) -> Result<User, CustomError> {
        let client = self.pool.get().await?;
        let user = db::queries::users::create_user(&client).await?;
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
        assert_ne!(user.password, password)
    }
}