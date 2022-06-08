-- Your SQL goes here
CREATE TABLE Class (
    class_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT
)
