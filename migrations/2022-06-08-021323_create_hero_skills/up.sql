-- Your SQL goes here
CREATE TABLE HeroSkill (
    hero_id BIGINT UNSIGNED NOT NULL,
    skill_id BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (hero_id, skill_id),
    FOREIGN KEY (hero_id) REFERENCES Hero(hero_id) ON DELETE CASCADE,
    FOREIGN KEY (skill_id) REFERENCES Skill(skill_id) ON DELETE CASCADE
)
