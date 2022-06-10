use crate::models::item_models::*;
use crate::repositories::user_repository::UserRepository;
use crate::schema::{Item, UserItem};
use diesel::prelude::*;

pub struct ItemRepository;

impl ItemRepository {
    // Query
    pub fn load_item_info(c: &MysqlConnection, load_item: LoadItemInfo) -> QueryResult<ItemInfo> {
        Item::table.find(load_item.item_id).first::<ItemInfo>(c)
    }

    fn load_item_price(c: &MysqlConnection, item_id: u64) -> QueryResult<u32> {
        Item::table
            .select(Item::price)
            .find(item_id)
            .first::<u32>(c)
    }

    pub fn load_all(c: &MysqlConnection) -> QueryResult<Vec<UserItemInfo>> {
        UserItem::table.limit(100).load::<UserItemInfo>(c)
    }

    pub fn find_by_user_id(
        c: &MysqlConnection,
        load_item: LoadItem,
    ) -> QueryResult<Vec<UserItemInfo>> {
        UserItem::table
            .filter(UserItem::user_id.eq(load_item.user_id))
            .get_results::<UserItemInfo>(c)
    }

    pub fn load_quantity_item_of_user(
        c: &MysqlConnection,
        user_id: u64,
        item_id: u64,
    ) -> QueryResult<u32> {
        UserItem::table
            .select(UserItem::quantity)
            .find((user_id, item_id))
            .first::<u32>(c)
    }

    // Update
    pub fn buy_new(conn: &MysqlConnection, new_item: BuyItem) -> Result<u32, String> {
        let item_quantity = new_item.quantity;
        let user_id = new_item.user_id;
        let item_id = new_item.item_id;

        // Get Item price
        let item_price = match Self::load_item_price(conn, item_id) {
            Ok(price) => price * item_quantity,
            Err(_) => return Err("Error getting item price".to_string()),
        };

        // Minus user's gold first
        UserRepository::pay_gold(conn, user_id, item_price)?;

        // create new hero
        match diesel::insert_into(UserItem::table)
            .values(new_item)
            .execute(conn)
        {
            Ok(_) => match Self::load_quantity_item_of_user(conn, user_id, item_id) {
                Ok(item_quantity) => Ok(item_quantity),
                Err(_) => Err("Bought successfully! Error in retrieve item's quantity".to_string()),
            },
            Err(_) => {
                // Create hero failed -> Plus user's gold back
                match UserRepository::receive_gold(conn, user_id, item_price) {
                    Ok(_) => Err("Bought failed! gave back gold to user".to_string()),
                    Err(_) => Err("SOS! User's gold is incorrect!".to_string()),
                }
            }
        }
    }
}
