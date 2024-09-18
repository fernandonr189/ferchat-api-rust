use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'a> {
    username: &'a str,
    password: &'a str,
    email: &'a str,
    id: i32,
    is_active: bool,
}
