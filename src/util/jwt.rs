use crate::models::response::Claims;
use chrono::Utc;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use jsonwebtoken::errors::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use std::env;

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS256);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let token = token.trim_start_matches("Bearer").trim();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }

}
