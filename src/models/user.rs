use mysql::prelude::*;
use mysql::*;
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

impl FromRow for User {
    fn from_row(row: Row) -> Self {
        let (id, username, password, email, is_active): (i32, String, String, String, bool) =
            mysql::from_row(row);
        User {
            username,
            id,
            password: Some(password),
            is_active,
            email,
        }
    }
    fn from_row_opt(row: Row) -> Result<User, mysql::FromRowError> {
        let (id, username, password, email, is_active) = mysql::from_row_opt(row)?;
        Ok(User {
            username,
            id,
            password: Some(password),
            is_active,
            email,
        })
    }
}
