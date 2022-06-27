#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;

use heroes_game_backend::database::{run_db_migrations, DbConnection};
use heroes_game_backend::routes::*;

use rocket::fairing::AdHoc;
use rocket::figment::{util::map, value};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::error::Error;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Config database
    dotenv().ok();
    let db: value::Map<_, value::Value> = map! {
        "url" => env::var("DATABASE_URL").unwrap().into(),
    };

    let figment = rocket::Config::figment().merge(("databases", map!["mysql_db" => db]));

    // CORS
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    // Launch Server
    let _rocket = rocket::custom(figment)
        .mount(
            "/auth",
            routes![
                auth_route::login,
                auth_route::logout,
                auth_route::register,
                auth_route::test_token,
                auth_route::get_gold,
            ],
        )
        .mount(
            "/hero",
            routes![
                hero_route::buy_new_hero,
                hero_route::load_heroes,
                hero_route::upgrade_hero,
            ],
        )
        .mount(
            "/item",
            routes![item_route::buy_new_item, item_route::load_items],
        )
        .mount("/battle", routes![battle_route::battle])
        .register(
            "/",
            catchers![catch_route::unauthorized, catch_route::not_found],
        )
        .attach(cors)
        .attach(DbConnection::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            run_db_migrations,
        ))
        .ignite()
        .await?
        .launch()
        .await;
    Ok(())
}
