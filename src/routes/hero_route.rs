use crate::auth::JWTAuth;
use crate::database::DbConnection;
use crate::models::hero_models::*;
use crate::repositories::HeroRepository;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[post("/hero/buy", format = "json", data = "<new_hero>")]
pub async fn buy_new_hero(
    _auth: JWTAuth,
    conn: DbConnection,
    new_hero: Json<NewHero>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        HeroRepository::buy_new(c, new_hero.into_inner())
            .map(|hero_info| json!(hero_info))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
