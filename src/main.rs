#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use std::env;

use heroes_game_backend::JWTAuth;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rocket::fairing::AdHoc;
use rocket::http::{Cookie, CookieJar, Method};
use rocket::serde::json::{json, Json, Value};
use rocket::Build;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_sync_db_pools::database;
use serde::Deserialize;
use sha2::Sha256;
use std::error::Error;

embed_migrations!();

#[database("mysql_db")]
struct DbConnection(diesel::MysqlConnection);

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

    // Construct Cookie: domain = empty, secure = true, SameSite = None
    // for cookies can be stored in web browser in localhost
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

async fn run_db_migrations(
    rocket: rocket::Rocket<Build>,
) -> Result<rocket::Rocket<Build>, rocket::Rocket<Build>> {
    DbConnection::get_one(&rocket)
        .await
        .expect("Failed to retrieve database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
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
