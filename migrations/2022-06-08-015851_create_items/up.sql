-- Your SQL goes here
CREATE TABLE Item (
    item_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price INTEGER UNSIGNED NOT NULL
)
