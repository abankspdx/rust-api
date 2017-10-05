/// A bunch of types that relate to Users
/// UserAuthenticate/UserRegister are mostly for handlers (Auth/Reg),
/// UserDisplay is more for a hypothetical UI. If a user was able to look up a different user
/// You wouldn't show them a user's salt/password
/// This could probably be made "better" by making some sort of generic struct

use schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserAuthenticate {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRegister {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct UserDisplay {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl UserDisplay {
    pub fn new(u: User) -> UserDisplay {
        UserDisplay {
            id: u.id,
            email: u.email,
            name: u.name
        }
    }
}