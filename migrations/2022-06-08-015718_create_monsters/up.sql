-- Your SQL goes here
CREATE TABLE IF NOT EXISTS
  Monster (
    monster_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    level TINYINT NOT NULL,
    win_rate INTEGER UNSIGNED NOT NULL
  );

INSERT INTO
  Monster (name, description, level, win_rate)
VALUES
  (
    'Demogorgon',
    'All Demogorgons are assumed to obey the Mind Flayer, and make up part of his hive mind. Under his influence, Demogorgons are murderous, violent and have limited intelligence',
    1,
    80
  );

INSERT INTO
  Monster (name, description, level, win_rate)
VALUES
  (
    'Mind Flayer',
    'The Mind Flayer is a massively powerful being of unknown origin. He wields supreme control of the Upside Down via a psychic link, controlling Demogorgons',
    2,
    60
  );

INSERT INTO
  Monster (name, description, level, win_rate)
VALUES
  (
    'Vecna',
    'Vecna prays on emotionally unstable teenagers. He mentally tortures them about their past and trauma for his own enjoyment, and then kills them by breaking their skeleton apart and taking their eyes',
    3,
    40
  );
