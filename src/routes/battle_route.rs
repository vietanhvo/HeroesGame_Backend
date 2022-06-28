use crate::auth::JWTAuth;
use crate::database::DbConnection;
use crate::models::battle_models::*;
use crate::repositories::BattleRepository;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[post("/", format = "json", data = "<battle>")]
pub async fn battle(
    auth: JWTAuth,
    conn: DbConnection,
    battle: Json<Battle>,
) -> Result<Value, status::Custom<Value>> {
    let user_id = auth.user.user_id;
    conn.run(move |c| {
        BattleRepository::battle(c, battle.into_inner(), user_id)
            .map(|battle_result| json!(battle_result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
