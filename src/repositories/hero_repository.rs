use crate::models::hero_models::*;
use crate::repositories::user_repository::UserRepository;
use crate::schema::Hero;
use diesel::prelude::*;

pub struct HeroRepository;

impl HeroRepository {
    // Query
    pub fn find_by_id(c: &MysqlConnection, id: u64) -> QueryResult<HeroInfo> {
        Hero::table.find(id).first::<HeroInfo>(c)
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
                match Self::find_by_id(conn, new_hero_id) {
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

    fn last_id(conn: &MysqlConnection) -> QueryResult<u64> {
        Hero::table
            .select(Hero::hero_id)
            .order(Hero::hero_id.desc())
            .first(conn)
    }
}
