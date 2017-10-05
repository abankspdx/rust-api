CREATE TABLE users (
    id serial primary key,
    name text not null,
    email text not null,
    password_hash text not null,
    salt text not null
);

ALTER TABLE users ADD UNIQUE (email);
ALTER TABLE users ADD UNIQUE (salt);
ALTER TABLE users OWNER TO postgres;