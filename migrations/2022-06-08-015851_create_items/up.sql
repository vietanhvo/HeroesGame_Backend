-- Your SQL goes here
CREATE TABLE IF NOT EXISTS
  Item (
    item_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price INTEGER UNSIGNED NOT NULL
  );

INSERT INTO
  Item (name, description, price)
VALUES
  (
    'Gem',
    'This item is used to upgrade your heroes',
    10
  );

INSERT INTO
  Item (name, description, price)
VALUES
  (
    'Core',
    'This item is used to protect your heroes stars when upgrading',
    1000
  );
