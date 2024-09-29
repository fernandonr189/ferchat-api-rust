use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FriendAccRequest {
    pub friend_id: i32,
    pub accept: bool,
}
