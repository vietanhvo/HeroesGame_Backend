use crate::models::item_models::*;
use crate::repositories::user_repository::UserRepository;
use crate::schema::{Item, UserItem};
use diesel::prelude::*;
use diesel::result::Error;

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

    pub fn find_by_user_id(c: &MysqlConnection, user_id: u64) -> QueryResult<Vec<UserItemInfo>> {
        UserItem::table
            .filter(UserItem::user_id.eq(user_id))
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

        // Get the current user's item quantity
        // If NotFound -> insert new data -> return
        let current_item_quantity = match Self::load_quantity_item_of_user(conn, user_id, item_id) {
            Ok(quantity) => quantity,
            Err(Error::NotFound) => {
                // Not found -> create new item for user
                match diesel::insert_into(UserItem::table)
                    .values(new_item)
                    .execute(conn)
                {
                    Ok(_) => return Ok(item_quantity),
                    Err(_) => {
                        // Create hero failed -> Plus user's gold back
                        match UserRepository::receive_gold(conn, user_id, item_price) {
                            Ok(_) => {
                                return Err(
                                    "Bought Item failed! gave back gold to user".to_string()
                                );
                            }
                            Err(_) => {
                                return Err("SOS! Bought Item failed! Error when gave back gold"
                                    .to_string());
                            }
                        }
                    }
                }
            }
            Err(_) => return Err("Retrieve current user's item failed!".to_string()),
        };

        // If Found -> update item quantity
        let new_item_quantity = current_item_quantity + item_quantity;
        match diesel::update(UserItem::table)
            .filter(UserItem::user_id.eq(user_id))
            .filter(UserItem::item_id.eq(item_id))
            .set(UserItem::quantity.eq(new_item_quantity))
            .execute(conn)
        {
            Ok(_) => Ok(new_item_quantity),
            Err(_) => {
                // Create hero failed -> Plus user's gold back
                match UserRepository::receive_gold(conn, user_id, item_price) {
                    Ok(_) => Err("Bought Item failed! gave back gold to user".to_string()),
                    Err(_) => Err("SOS! Bought Item failed! Error when gave back gold".to_string()),
                }
            }
        }
    }

    pub fn use_item(conn: &MysqlConnection, use_item: UseItem) -> Result<u32, String> {
        let item_quantity = use_item.quantity;
        let user_id = use_item.user_id;
        let item_id = use_item.item_id;

        // Check user item quantity
        let user_item_quantity = match Self::load_quantity_item_of_user(conn, user_id, item_id) {
            Ok(item_quantity) => item_quantity,
            Err(_) => return Err("Use failed! Error in retrieve item's quantity".to_string()),
        };

        if item_quantity > user_item_quantity {
            Err("Use failed! Not enough item".to_string())
        } else {
            // Minus user's item quantity
            match diesel::update(UserItem::table)
                .filter(UserItem::user_id.eq(user_id))
                .filter(UserItem::item_id.eq(item_id))
                .set(UserItem::quantity.eq(user_item_quantity - item_quantity))
                .execute(conn)
            {
                Ok(_) => Ok(user_item_quantity - item_quantity),
                Err(_) => Err("Use failed! Error in update item's quantity".to_string()),
            }
        }
    }

    pub fn increase_item_quantity(
        conn: &MysqlConnection,
        use_item: UseItem,
    ) -> Result<u32, String> {
        let item_quantity = use_item.quantity;
        let user_id = use_item.user_id;
        let item_id = use_item.item_id;

        let current_item_quantity = match Self::load_quantity_item_of_user(conn, user_id, item_id) {
            Ok(item_quantity) => item_quantity,
            Err(_) => return Err("Use failed! Error in retrieve item's quantity".to_string()),
        };

        match diesel::update(UserItem::table)
            .filter(UserItem::user_id.eq(user_id))
            .filter(UserItem::item_id.eq(item_id))
            .set(UserItem::quantity.eq(current_item_quantity + item_quantity))
            .execute(conn)
        {
            Ok(_) => Ok(current_item_quantity + item_quantity),
            Err(_) => Err("Increase Item failed! Error in update item's quantity".to_string()),
        }
    }
}
