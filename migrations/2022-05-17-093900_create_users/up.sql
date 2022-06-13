-- Your SQL goes here
CREATE TABLE IF NOT EXISTS
  User (
    user_id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    date_of_birth DATE NOT NULL,
    gender CHAR(1) NOT NULL,
    password VARCHAR(255) NOT NULL,
    gold INTEGER UNSIGNED NOT NULL DEFAULT 100000
  )
