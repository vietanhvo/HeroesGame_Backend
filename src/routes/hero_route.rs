use crate::auth::JWTAuth;
use crate::database::DbConnection;
use crate::models::hero_models::*;
use crate::repositories::HeroRepository;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[get("/load", format = "json")]
pub async fn load_heroes(
    auth: JWTAuth,
    conn: DbConnection,
) -> Result<Value, status::Custom<Value>> {
    let user_id = auth.user.user_id;
    conn.run(move |c| {
        HeroRepository::find_by_user_id(c, user_id)
            .map(|heroes| json!(heroes))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/buy", format = "json", data = "<new_hero>")]
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

#[post("/upgrade", format = "json", data = "<upgrade_hero>")]
pub async fn upgrade_hero(
    _auth: JWTAuth,
    conn: DbConnection,
    upgrade_hero: Json<UpgradeHero>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        HeroRepository::upgrade(c, upgrade_hero.into_inner())
            .map(|hero_info| json!(hero_info))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/battle", format = "json", data = "<battle>")]
pub async fn battle(
    auth: JWTAuth,
    conn: DbConnection,
    battle: Json<Battle>,
) -> Result<Value, status::Custom<Value>> {
    let user_id = auth.user.user_id;
    conn.run(move |c| {
        HeroRepository::battle(c, battle.into_inner(), user_id)
            .map(|hero_info| json!(hero_info))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
