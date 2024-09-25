use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<'a> {
    pub error_code: Option<i32>,
    pub message: &'a str,
}
