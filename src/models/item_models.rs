use crate::schema::UserItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserItemInfo {
    pub user_id: u64,
    pub item_id: u64,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct ItemInfo {
    pub item_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub price: u32,
}

// Model for buy new item
#[derive(Insertable, Deserialize)]
#[table_name = "UserItem"]
pub struct BuyItem {
    pub user_id: u64,
    pub item_id: u64,
    pub quantity: u32,
}

// // Model for load items
// #[derive(Queryable, Deserialize)]
// pub struct LoadItem {
//     pub user_id: u64,
// }

// Model for load Item info
#[derive(Queryable, Deserialize)]
pub struct LoadItemInfo {
    pub item_id: u64,
}

// Model for use Item
#[derive(Deserialize, Clone, Copy)]
pub struct UseItem {
    pub item_id: u64,
    pub user_id: u64,
    pub quantity: u32,
}
