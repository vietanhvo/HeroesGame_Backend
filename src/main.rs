#[macro_use]
extern crate rocket;

use heroes_game_backend::BasicAuth;
use rocket::serde::json::{serde_json::json, Value};

#[get("/test")]
async fn test(_auth: BasicAuth) -> Value {
    json!("Hello, Rustaceans!")
}

#[catch(401)]
fn unauthorized(_req: &rocket::Request) -> Value {
    json!("Unauthorized")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![test])
        .register("/", catchers![unauthorized])
        .launch()
        .await;
    Ok(())
}
