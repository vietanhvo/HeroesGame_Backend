use super::monster_repository::MonsterRepository;
use super::hero_repository::HeroRepository;
use super::user_repository::UserRepository;
use crate::models::battle_models::*;
use crate::schema::Hero;
use diesel::prelude::*;
use rand::Rng;

pub struct BattleRepository;

impl BattleRepository {
    pub fn battle(conn: &MysqlConnection, battle: Battle, user_id: u64) -> Result<u8, String> {
        let hero_id = battle.hero_id;
        let monster_id = battle.monster_id;

        // Retrieve monster win rate
        let win_rate = match MonsterRepository::get_win_rate(conn, monster_id) {
            Ok(win_rate) => win_rate,
            Err(_) => return Err("Error in retrieve monster win rate".to_string()),
        };

        // Retrieve hero exp
        let current_hero_exp = match HeroRepository::find_by_hero_id(conn, hero_id) {
            Ok(hero) => hero.experience,
            Err(_) => return Err("Error in retrieve hero exp".to_string()),
        };

        // Generate random number from 0 to 100
        let mut rng = rand::thread_rng();
        let random_win_rate: u32 = rng.gen_range(0..=100);

        // Battle
        // Win
        if random_win_rate <= win_rate {
            // Increase hero exp
            match diesel::update(Hero::table)
                .filter(Hero::hero_id.eq(hero_id))
                .set(Hero::experience.eq(current_hero_exp + 100))
                .execute(conn)
            {
                Ok(_) => {
                    // Increase user's gold
                    match UserRepository::receive_gold(conn, user_id, 100) {
                        Ok(_) => return Ok(1),
                        Err(_) => {
                            match diesel::update(Hero::table)
                                .filter(Hero::hero_id.eq(hero_id))
                                .set(Hero::experience.eq(current_hero_exp))
                                .execute(conn)
                            {
                                Ok(_) => return Err("Error in update gold!".to_string()),
                                Err(_) => {
                                    return Err(
                                        "SOS! hero exp increased but user gold is not updated"
                                            .to_string(),
                                    )
                                }
                            };
                        }
                    };
                }
                Err(_) => return Err("Error in increase hero exp".to_string()),
            };
        }
        // Lose
        Ok(0)
    }

}
