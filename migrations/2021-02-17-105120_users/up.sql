-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    creation_date DATE NOT NULL,
    creation_time TIME NOT NULL
)
