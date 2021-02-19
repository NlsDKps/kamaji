-- Your SQL goes here
CREATE TABLE auth_infos (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    password_hash TEXT NOT NULL,
    last_changed_date DATE NOT NULL,
    last_changed_time TIME NOT NULL
)
