use super::item_repository::ItemRepository;
use super::monster_repository::MonsterRepository;
use super::user_repository::UserRepository;
use crate::models::hero_models::*;
use crate::models::item_models::UseItem;
use crate::schema::Hero;
use diesel::prelude::*;
use rand::Rng;

pub struct HeroRepository;

impl HeroRepository {
    // Query
    pub fn load_all(c: &MysqlConnection) -> QueryResult<Vec<HeroInfo>> {
        Hero::table.limit(100).load::<HeroInfo>(c)
    }

    pub fn find_by_hero_id(c: &MysqlConnection, hero_id: u64) -> QueryResult<HeroInfo> {
        Hero::table.find(hero_id).first::<HeroInfo>(c)
    }

    pub fn find_by_user_id(c: &MysqlConnection, user_id: u64) -> QueryResult<Vec<HeroInfo>> {
        Hero::table
            .filter(Hero::user_id.eq(user_id))
            .get_results::<HeroInfo>(c)
    }

    // Update
    pub fn buy_new(conn: &MysqlConnection, new_hero: NewHero) -> Result<HeroInfo, String> {
        let hero_price = new_hero.price;
        let user_id = new_hero.user_id;

        // Minus user's gold first
        UserRepository::pay_gold(conn, user_id, hero_price)?;

        // create new hero
        match diesel::insert_into(Hero::table)
            .values(new_hero)
            .execute(conn)
        {
            Ok(_) => {
                let new_hero_id = match Self::last_id(conn) {
                    Ok(hero_id) => hero_id,
                    Err(_) => {
                        return Err("Bought successfully! Error in retrieve new hero id".to_string())
                    }
                };
                match Self::find_by_hero_id(conn, new_hero_id) {
                    Ok(hero) => Ok(hero),
                    Err(_) => {
                        Err("Bought successfully! Error in retrieve new hero info".to_string())
                    }
                }
            }
            Err(_) => {
                // Create hero failed -> Plus user's gold back
                match UserRepository::receive_gold(conn, user_id, hero_price) {
                    Ok(_) => Err("Create hero failed! gave back gold to user".to_string()),
                    Err(_) => Err("SOS! User's gold is incorrect!".to_string()),
                }
            }
        }
    }

    pub fn upgrade(conn: &MysqlConnection, upgrade_hero: UpgradeHero) -> Result<u8, String> {
        let upgrade_percentage = [45, 30, 15, 19, 7];

        let hero_id = upgrade_hero.hero_id;
        let user_id = upgrade_hero.user_id;

        // Retrieve current hero stars
        let hero_star = match Self::find_by_hero_id(conn, hero_id) {
            Ok(hero) => hero.stars,
            Err(_) => return Err("Error in retrieve hero star".to_string()),
        };

        let needed_gem_quantity = hero_star as u32 * 100;

        // Generate random number from 0 to 100
        let mut rng = rand::thread_rng();
        let random_percent_data: u32 = rng.gen_range(0..=100);

        // Minus user gem
        let use_gem = UseItem {
            user_id,
            item_id: 1,
            quantity: needed_gem_quantity,
        };
        ItemRepository::use_item(conn, use_gem)?;

        // Upgrade hero's stars
        let upgrade_percentage_data = upgrade_percentage[(hero_star - 1) as usize];
        if random_percent_data <= upgrade_percentage_data {
            // Success
            let new_hero_star = hero_star + 1;
            match diesel::update(Hero::table)
                .filter(Hero::hero_id.eq(hero_id))
                .set(Hero::stars.eq(new_hero_star))
                .execute(conn)
            {
                Ok(_) => Ok(new_hero_star),
                Err(_) => {
                    // Upgrade hero's stars failed -> Plus user's gem back
                    match ItemRepository::increase_item_quantity(conn, use_gem) {
                        Ok(_) => {
                            Err("Upgrade hero's stars failed! gave back gem to user".to_string())
                        }
                        Err(_) => Err("SOS! User's gem is incorrect!".to_string()),
                    }
                }
            }
        } else {
            // Failed
            Ok(hero_star)
        }
    }

    pub fn battle(conn: &MysqlConnection, battle: Battle, user_id: u64) -> Result<u8, String> {
        let hero_id = battle.hero_id;
        let monster_id = battle.monster_id;

        // Retrieve monster win rate
        let win_rate = match MonsterRepository::get_win_rate(conn, monster_id) {
            Ok(win_rate) => win_rate,
            Err(_) => return Err("Error in retrieve monster win rate".to_string()),
        };

        // Retrieve hero exp
        let current_hero_exp = match Self::find_by_hero_id(conn, hero_id) {
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

    fn last_id(conn: &MysqlConnection) -> QueryResult<u64> {
        Hero::table
            .select(Hero::hero_id)
            .order(Hero::hero_id.desc())
            .first(conn)
    }
}
