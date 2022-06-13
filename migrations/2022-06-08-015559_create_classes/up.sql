-- Your SQL goes here
CREATE TABLE IF NOT EXISTS
  Class (
    class_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT
  );

INSERT INTO
  Class (name, description)
VALUES
  ('Shooter', 'A class has strong attack stats');

INSERT INTO
  Class (name, description)
VALUES
  ('Tanker', 'A class has strong defense stats');
