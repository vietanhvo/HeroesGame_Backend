use serde::Deserialize;

// Model for battle
#[derive(Deserialize)]
pub struct Battle {
    pub hero_id: u64,
    pub monster_id: u64,
}
