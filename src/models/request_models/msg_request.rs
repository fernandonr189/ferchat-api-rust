use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MsgRequest {
    pub destination: i32,
    pub msg: String,
}
