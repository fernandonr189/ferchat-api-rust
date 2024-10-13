use mysql::prelude::FromRow;
use mysql::{from_row, from_row_opt, FromRowError, Row};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub id: i32,
    pub password: Option<String>,
    pub is_active: bool,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSimplified {
    pub username: String,
    pub id: i32,
}

impl FromRow for UserSimplified {
    fn from_row(row: Row) -> Self {
        let (id, username): (i32, String) = from_row(row);
        UserSimplified { username, id }
    }
    fn from_row_opt(row: Row) -> Result<UserSimplified, FromRowError> {
        let (id, username) = from_row_opt(row)?;
        Ok(UserSimplified { username, id })
    }
}

impl FromRow for User {
    fn from_row(row: Row) -> Self {
        let (id, username, password, email, is_active): (i32, String, String, String, bool) =
            from_row(row);
        User {
            username,
            id,
            password: Some(password),
            is_active,
            email,
        }
    }
    fn from_row_opt(row: Row) -> Result<User, FromRowError> {
        let (id, username, password, email, is_active) = from_row_opt(row)?;
        Ok(User {
            username,
            id,
            password: Some(password),
            is_active,
            email,
        })
    }
}
