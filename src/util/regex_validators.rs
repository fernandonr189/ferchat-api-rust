use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    let re = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    re.is_match(email)
}

pub fn validate_username(username: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9._]{1,28}[a-zA-Z0-9])?$").unwrap();
    re.is_match(username)
}

pub fn validate_password_security(password: &str) -> bool {
    password.len() > 8
        && password.chars().any(|c| c.is_uppercase())
        && password.chars().any(|c| c.is_lowercase())
        && password.chars().any(|c| c.is_numeric())
        && password.chars().any(|c| r"@$!%*?&".contains(c))
}
