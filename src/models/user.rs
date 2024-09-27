use crate::models::traits::Insertable;
use mysql::prelude::*;
use mysql::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub password: Option<String>,
    pub id: i32,
    pub is_active: bool,
    pub email: String,
}

impl Insertable for User {
    fn insert(&self, conn: &mut PooledConn) -> Result<bool, mysql::Error> {
        match conn.exec_drop(
            r"INSERT INTO users (username, password, email, is_active) VALUES (:username, :password, :email, :is_active)",
            params! {
                "username" => &self.username,
                "password" => &self.password,
                "email" => &self.email,
                "is_active" => self.is_active,
            },
        ) {
            Ok(_) => {
                Ok(true)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}

impl FromRow for User {
    fn from_row(row: Row) -> Self {
        let (id, username, email, is_active): (i32, String, String, bool) = mysql::from_row(row);
        User {
            username,
            password: None,
            id,
            is_active,
            email,
        }
    }
    fn from_row_opt(row: Row) -> Result<User, mysql::FromRowError> {
        let (id, username, email, is_active) = mysql::from_row_opt(row)?;
        Ok(User {
            username,
            password: None,
            id,
            is_active,
            email,
        })
    }
}
