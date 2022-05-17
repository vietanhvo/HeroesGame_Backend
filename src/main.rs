#[macro_use]
extern crate rocket;

use rocket::serde::json::{serde_json::json, Value};

#[get("/test")]
async fn test() -> Value {
    json!("Hello, Rustaceans!")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![test]).launch().await;
    Ok(())
}
