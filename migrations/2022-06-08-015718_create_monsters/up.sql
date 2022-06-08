-- Your SQL goes here
CREATE TABLE Monster (
    monster_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    level TINYINT NOT NULL,
    win_rate FLOAT NOT NULL
)
