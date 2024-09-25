use crate::models::traits::Insertable;
use mysql::prelude::*;
use mysql::*;
use rocket::serde::{json::Json, Deserialize, Serialize};

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
                println!("User created");
                Ok(true)
            },
            Err(_) => {
                println!("user not created");
                Ok(false)
            }
        }
    }
}

impl FromRow for User {
    fn from_row(row: Row) -> Self {
        let (id, username, password, email, is_active) = mysql::from_row(row); // Use from_row helper
        User {
            username,
            password,
            id,
            is_active,
            email,
        }
    }
    fn from_row_opt(row: Row) -> Result<User, mysql::FromRowError> {
        let (id, username, password, email, is_active) = mysql::from_row_opt(row)?;
        Ok(User {
            username,
            password,
            id,
            is_active,
            email,
        })
    }
}

pub fn from_json(data: Json<User>) -> User {
    let new_user = User {
        username: data.0.username,
        password: data.0.password,
        id: data.0.id,
        is_active: data.0.is_active,
        email: data.0.email,
    };
    return new_user;
}
