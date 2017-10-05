/// Service functions around creating and getting users
/// I made a point to have every service return `Option<T>`
/// Just for consistently.

extern crate diesel;
use diesel::*;
use database::establish_connection;
use types::user::{User, UserRegister, NewUser, UserAuthenticate};
use schema::users::dsl::*;
use super::password;


pub fn get_users() -> Option<Vec<User>> {
    let connection = establish_connection();
    let results = users.load::<User>(&connection).expect("Error loading users");
    return Some(results)
}

pub fn get_user(user_id: i32) -> Option<User> {
    let connection = establish_connection();
    let results = users.filter(id.eq(user_id)).limit(1).load::<User>(&connection).expect("Error finding user");
    if results.len() == 0 {
        return None
    }
    return Some(results[0].clone())
}

pub fn authenticate(auth_user: UserAuthenticate) -> Option<String> {
    let connection = establish_connection();
    let results = users.filter(email.eq(auth_user.email))
        .limit(1)
        .load::<User>(&connection)
        .expect("Error finding user");

    let found = results[0].clone();
    if password::verify_password(&found.password_hash, &auth_user.password, &found.salt) {
        return Some(super::jwt::encode(found))
    } else {
        return None
    }
}

pub fn create_user(user: UserRegister) -> Result<User, String> {
    use schema::users;
    let connection = establish_connection();
    let user_salt = password::generate_salt();
    let user_hash = password::hash_password(&user.password, &user_salt).unwrap();
    let new_user = NewUser {
        name: user.name,
        email: user.email,
        password_hash: user_hash,
        salt: user_salt
    };

    match diesel::insert(&new_user)
        .into(users::table)
        .get_result(&connection) {
        Ok(t) => return Ok(t),
        Err(e) => return Err(e.to_string())
    }

}
