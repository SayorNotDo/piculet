use crypto_utils::sha::{Algorithm, CryptographicHash};
use serde::Serialize;
use uuid::Uuid;
use crate::errors::CustomError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub uuid: Uuid,
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
            Uuid::new_v4()
        } else {
            Uuid::nil()
        };

        Self {
            uuid,
            username,
            password,
        }
    }
}

pub struct UserDao {}

impl UserDao {
    pub async fn create_user(User: User) -> Result<(), CustomError> {
        println!("{:?}", User);
        Ok(())
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

    #[tokio::test]
    async fn test_create_user() {
        let username = "username";
        let password = "password";

        let db_url = std::env::var("DATABASE_URL").unwrap();

        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();

        let user = User::new(username, password, true);
        let result = db::queries::users::insert_user()
        .bind(&client, &user.uuid, &user.username, &user.password).await.unwrap();
        println!("---------------------{:?}", result);

        assert_eq!(1, 1)
    }
}