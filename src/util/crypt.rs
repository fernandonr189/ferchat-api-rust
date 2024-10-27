use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> String {
    hash(password, 10).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}
