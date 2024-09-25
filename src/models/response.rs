use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Data<T> {
    Model(T),
    None,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<'a, T> {
    pub error_code: Option<i32>,
    pub message: &'a str,
    pub data: Option<Data<T>>,
}
