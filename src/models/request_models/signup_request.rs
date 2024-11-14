use crate::util::regex_validators;

use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl SignupRequest {
    pub fn verify_email(&self) -> bool {
        regex_validators::validate_email(&self.email)
    }
    pub fn verify_username(&self) -> bool {
        regex_validators::validate_username(&self.username)
    }
    pub fn verify_password(&self) -> bool {
        regex_validators::validate_password_security(&self.password)
    }
}
