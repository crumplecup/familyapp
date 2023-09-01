DROP TABLE IF EXISTS families;

CREATE TABLE families (
    family_id serial PRIMARY KEY,
    name TEXT NOT NULL
);

DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id serial PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);
