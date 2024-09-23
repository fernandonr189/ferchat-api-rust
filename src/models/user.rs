use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub id: i32,
    pub is_active: bool,
    pub email: &'a str,
}

pub fn from_json<'r>(data: Json<User<'r>>) -> User<'r> {
    let new_user = User {
        username: data.0.username,
        password: data.0.password,
        id: data.0.id,
        is_active: data.0.is_active,
        email: data.0.email,
    };
    return new_user;
}
