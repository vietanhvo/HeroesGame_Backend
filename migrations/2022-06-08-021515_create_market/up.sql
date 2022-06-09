-- Your SQL goes here
CREATE TABLE IF NOT EXISTS
  Market (
    transaction_id SERIAL PRIMARY KEY,
    seller_id BIGINT UNSIGNED NOT NULL,
    buyer_id BIGINT UNSIGNED NOT NULL,
    hero_id BIGINT UNSIGNED NOT NULL,
    post_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP NOT NULL,
    price INTEGER UNSIGNED NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT "pending",
    FOREIGN KEY (seller_id) REFERENCES User(user_id),
    FOREIGN KEY (buyer_id) REFERENCES User(user_id),
    FOREIGN KEY (hero_id) REFERENCES Hero(hero_id) ON DELETE CASCADE
  )
