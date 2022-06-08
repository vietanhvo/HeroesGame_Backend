-- Your SQL goes here
CREATE TABLE Hero (
    hero_id SERIAL PRIMARY KEY,
    user_id BIGINT UNSIGNED NOT NULL,
    class_id BIGINT UNSIGNED NOT NULL,
    weapon_id BIGINT UNSIGNED,
    name VARCHAR(255) NOT NULL,
    level TINYINT UNSIGNED NOT NULL DEFAULT 1,
    stars TINYINT UNSIGNED NOT NULL DEFAULT 1,
    price FLOAT NOT NULL,
    experience FLOAT UNSIGNED NOT NULL,
    energy INTEGER UNSIGNED NOT NULL DEFAULT 100,
    last_battle_time TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES User(user_id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES Class(class_id),
    FOREIGN KEY (weapon_id) REFERENCES Weapon(weapon_id)
)
