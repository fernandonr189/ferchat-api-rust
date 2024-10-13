use mysql::prelude::FromRow;
use mysql::{from_row, from_row_opt, FromRowError, Row};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Friendship {
    pub user_id: i32,
    pub friend_id: i32,
    pub sender_id: i32,
    pub status: i32,
    pub created_at: String,
}

impl FromRow for Friendship {
    fn from_row(row: Row) -> Self {
        let (user_id, friend_id, sender_id, status, created_at): (i32, i32, i32, i32, String) =
            from_row(row);
        Friendship {
            user_id,
            friend_id,
            sender_id,
            status,
            created_at,
        }
    }
    fn from_row_opt(row: Row) -> Result<Friendship, FromRowError> {
        let (user_id, friend_id, sender_id, status, created_at) = from_row_opt(row)?;
        Ok(Friendship {
            user_id,
            friend_id,
            sender_id,
            status,
            created_at,
        })
    }
}
