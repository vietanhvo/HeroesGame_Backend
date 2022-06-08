-- Your SQL goes here
CREATE TABLE User (
    user_id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    date_of_birth DATE NOT NULL,
    gender ENUM("m", "f", "o") NOT NULL,
    password VARCHAR(255) NOT NULL,
    gold FLOAT NOT NULL DEFAULT 10000
)
