use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FriendReqRequest {
    pub friend_id: i32,
}
