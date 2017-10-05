/// The database schema to verify the Diesel code with at compile time
/// This could be replaced with `infer_schema!(db_url)` if a user isn't using docker
/// To get docker-compose to work, this schema had to be set without access to the actual DB
table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        salt -> Text,
    }
}
