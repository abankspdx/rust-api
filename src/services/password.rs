/// Functions to generate salt, hash and verify passwords

use rand;
use rand::Rng;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn generate_salt() -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .take(20)
        .collect::<String>()
}

pub fn hash_password(password: &str, salt: &str) -> Option<String> {
    let pass = format!("{}:{}", salt, password);
    let results =  hash(&pass, DEFAULT_COST);
    match results {
        Ok(password_hash) => return Some(password_hash),
        Err(_) => return None
    }
}

pub fn verify_password(password_hash: &str, password: &str, salt: &str) -> bool {
    let formatted = format!("{}:{}", salt, password);
    match verify(&formatted, &password_hash) {
        Ok(t) => return t,
        Err(_) => return false
    }
}

#[cfg(test)]
mod tests {
    use super::generate_salt;
    use super::hash_password;
    use super::verify_password;

    #[test]
    fn test_salt() {
        let salt = generate_salt();
        assert_eq!(20, salt.len());
    }

    #[test]
    fn test_hash_and_validate() {
        let salt = generate_salt();
        let pass = "alex";
        let hash = hash_password(pass, &salt).unwrap();
        let verified = verify_password(&hash, &pass, &salt);
        assert_eq!(verified, true);
    }
}