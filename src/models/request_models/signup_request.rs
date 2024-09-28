use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}
