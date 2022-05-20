#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

use heroes_game_backend::JWTAuth;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rocket::http::{Cookie, CookieJar, Method};
use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;
use sha2::Sha256;

use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::error::Error;

#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[post("/login", format = "json", data = "<user_auth>")]
async fn login(user_auth: Json<User>, cookies: &CookieJar<'_>) -> Value {
    dotenv().ok();
    let user_auth = user_auth.into_inner();

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(env::var("JWT_SECRET").unwrap().as_bytes()).unwrap();
    let jwt_token = user_auth.email.sign_with_key(&key).unwrap();

    // Construct Cookie with domain = empty, secure = true, SameSite = None for cookies in web
    // browser with localhost
    let cookie = Cookie::build("token", jwt_token)
        .path("/")
        .secure(true)
        .same_site(rocket::http::SameSite::None)
        .http_only(true)
        .finish();

    cookies.add_private(cookie);
    // cookies.add_private(Cookie::new("token", jwt_token));

    json!({
        "status": "success",
        "message": "Logged in successfully",
    })
}

#[get("/test")]
async fn test(_auth: JWTAuth) -> Value {
    json!("Hello, Rustaceans!")
}

#[catch(401)]
fn unauthorized(_req: &rocket::Request) -> Value {
    json!("Unauthorized")
}

#[catch(404)]
fn not_found(_req: &rocket::Request) -> Value {
    json!("Not found")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    let _rocket = rocket::build()
        .mount("/", routes![login, test])
        .register("/", catchers![unauthorized, not_found])
        .attach(cors)
        .launch()
        .await;
    Ok(())
}
