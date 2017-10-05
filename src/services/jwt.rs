/// Functions and tests around creating/validating JWT tokens.
/// Validate Header kind of breaks convention in relation to HTTP,
/// But I didn't want to create a handler that never actually handled a route.
/// More things could be added to the payload pretty easily, at the time of writing
/// this I just didn't have anything else easily implementable to add.

extern crate frank_jwt;

use frank_jwt::{Header, Payload, Algorithm};
use frank_jwt::encode as jwt_encode;
use frank_jwt::decode as jwt_decode;
use dotenv::dotenv;
use std::env;
use serde_json;

use types::user::User;

type Token = String;

pub fn validate_header(val: Vec<u8>) -> Option<User> {
    let auth_string = String::from_utf8(val).unwrap();
    let mut iter = auth_string.split_whitespace();
    let first = iter.next();
    if !check_first(first) {
        return None;
    }
    let second = iter.next();
    return check_second(second);
}

pub fn encode(u: User) -> Token {
    dotenv().ok();
    let secret = env::var("JWT_SECRET")
        .expect("JWT Secret must be set");

    let mut payload = Payload::new();
    payload.insert("principal".to_string(), json!(u).to_string());
    let header = Header::new(Algorithm::HS256);

    return jwt_encode(header, secret.to_string(), payload.clone());
}

pub fn decode(token: Token) -> Option<User> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET")
        .expect("JWT Secret must be set");

    let result = jwt_decode(token, secret.to_string(), Algorithm::HS256);
    if result.is_err() {
        return None
    }

    let (_, contents) = result.unwrap();
    if !contents.contains_key("principal") {
        return None
    }

    let principal_string = contents.get("principal").unwrap();
    let user_result = serde_json::from_str(principal_string);

    if user_result.is_err() {
        return None
    }

    return Some(user_result.unwrap())
}

fn check_first(first: Option<&str>) -> bool {
    if first == None {
        return false
    }

    let value = first.unwrap().to_string();
    if value != "Bearer".to_string() {
        return false
    }
    return true
}

fn check_second(second: Option<&str>) -> Option<User> {
    if second == None {
        return None
    }

    let token = second.unwrap().to_string();
    return decode(token);
}

#[cfg(test)]
mod tests {
    use super::decode;
    use super::encode;
    use super::check_first;
    use super::check_second;
    use super::User;

    #[test]
    fn test_encode_and_decode() {
        let test_user = User {
            id: 5,
            email: "test@user.com".to_string(),
            name: "Test User".to_string(),
            password_hash: "abc".to_string(),
            salt: "123".to_string()
        };
        let encoded = encode(test_user.clone());
        let decoded = decode(encoded).unwrap();
        assert_eq!(test_user.id, decoded.id);
        assert_eq!(test_user.email, decoded.email);
    }

    #[test]
    fn test_check_first() {
        let empty = "".to_string();
        let wrong1 = "Bearer".to_string();
        let wrong2 = "Alex".to_string();

        let mut splitted_empty = empty.split_whitespace();
        let mut splitted_wrong1 = wrong1.split_whitespace();
        let mut splitted_wrong2 = wrong2.split_whitespace();

        assert_eq!(check_first(splitted_empty.next()), false);
        assert_eq!(check_first(splitted_wrong2.next()), false);
        assert_eq!(check_first(splitted_wrong1.next()), true);
    }

    #[test]
    fn test_check_second() {
        let test_user = User {
            id: 5,
            email: "test@user.com".to_string(),
            name: "Test User".to_string(),
            password_hash: "abc".to_string(),
            salt: "123".to_string()
        };
        let encoded = encode(test_user.clone());
        let token = format!("Bearer {}", encoded);
        let mut split = token.split_whitespace();
        split.next(); // destroy Bearer
        let output_user = check_second(split.next());

        if output_user.is_none() {
            assert!(false);
        }

        let u = output_user.unwrap();
        assert_eq!(u.id, test_user.id);
        assert_eq!(u.email, test_user.email);
    }
}