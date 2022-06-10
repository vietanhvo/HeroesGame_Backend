-- Your SQL goes here
CREATE TABLE IF NOT EXISTS
  Skill (
    skill_id SERIAL PRIMARY KEY,
    class_id BIGINT UNSIGNED NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    level_min TINYINT UNSIGNED NOT NULL,
    star_min TINYINT UNSIGNED NOT NULL,
    win_rate_increase INTEGER UNSIGNED NOT NULL,
    price INTEGER UNSIGNED NOT NULL,
    FOREIGN KEY (class_id) REFERENCES Class(class_id)
  )
