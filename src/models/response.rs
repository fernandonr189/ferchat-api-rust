use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<'a> {
    pub status_code: i32,
    pub message: &'a str,
}
