use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> String {
    let hashed_password = hash(password, 10).unwrap();
    return hashed_password;
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let is_valid = verify(password, hash).unwrap();
    return is_valid;
}
