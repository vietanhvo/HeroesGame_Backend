use crate::schema::Hero;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct HeroInfo {
    pub hero_id: u64,
    pub user_id: u64,
    pub class_id: u64,
    pub weapon_id: Option<u64>,
    pub name: String,
    pub level: u8,
    pub stars: u8,
    pub price: u32,
    pub experience: u32,
    pub energy: u32,
    pub last_battle_time: Option<NaiveDateTime>,
}

// Model for buy new hero
#[derive(Insertable, Deserialize)]
#[table_name = "Hero"]
pub struct NewHero {
    pub user_id: u64,
    pub class_id: u64,
    pub name: String,
    pub price: u32,
}

// Model for load heroes
#[derive(Queryable, Deserialize)]
pub struct LoadHero {
    pub user_id: u64,
}

// Model for update hero
#[derive(Deserialize)]
pub struct UpgradeHero {
    pub user_id: u64,
    pub hero_id: u64,
    pub needed_item_quantity: u32,
}
