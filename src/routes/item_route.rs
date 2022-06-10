use crate::auth::JWTAuth;
use crate::database::DbConnection;
use crate::models::item_models::*;
use crate::repositories::ItemRepository;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[post("/load", format = "json", data = "<load_item>")]
pub async fn load_items(
    _auth: JWTAuth,
    conn: DbConnection,
    load_item: Json<LoadItem>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        ItemRepository::find_by_user_id(c, load_item.into_inner())
            .map(|items| json!(items))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/buy", format = "json", data = "<new_item>")]
pub async fn buy_new_item(
    _auth: JWTAuth,
    conn: DbConnection,
    new_item: Json<BuyItem>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        ItemRepository::buy_new(c, new_item.into_inner())
            .map(|item_quantity| json!(item_quantity))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
