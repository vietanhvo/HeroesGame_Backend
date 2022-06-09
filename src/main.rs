#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use rocket::http::Status;
use std::env;

use heroes_game_backend::database::DbConnection;
use heroes_game_backend::models::*;
use heroes_game_backend::repositories::*;
use heroes_game_backend::JWTAuth;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rocket::fairing::AdHoc;
use rocket::figment::{util::map, value};
use rocket::http::{Cookie, CookieJar, Method};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket::Build;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use sha2::Sha256;
use std::error::Error;

embed_migrations!();

#[post("/api/login", format = "json", data = "<user_auth>")]
async fn login(
    conn: DbConnection,
    user_auth: Json<user_models::AuthUser>,
    cookies: &CookieJar<'_>,
) -> Result<Value, status::Custom<Value>> {
    let user_auth = user_auth.into_inner();
    let remember = user_auth.remember;

    let auth_result = conn
        .run(|c| {
            UserRepository::verify_account(c, user_auth)
                .map(|user_info| user_info)
                .map_err(|e| status::Custom(Status::Unauthorized, json!(e.to_string())))
        })
        .await;

    match auth_result {
        Ok(user_info) => {
            dotenv().ok();
            // Generate JWT Token
            let key: Hmac<Sha256> =
                Hmac::new_from_slice(env::var("JWT_SECRET").unwrap().as_bytes()).unwrap();
            let jwt_token = user_info.email.clone().sign_with_key(&key).unwrap();

            // Construct Cookie: domain = empty, secure = true, SameSite = None
            // for cookies can be stored in web browser in localhost
            let mut cookie = Cookie::build("token", jwt_token)
                .path("/")
                .secure(true)
                .same_site(rocket::http::SameSite::None)
                .http_only(true);

            if !remember {
                cookie = cookie.expires(None);
            }

            cookies.add_private(cookie.finish());
            Ok(json!(user_info))
        }
        Err(e) => Err(e),
    }
}

#[get("/api/logout", format = "json")]
async fn logout(cookies: &CookieJar<'_>) -> Result<Value, status::Custom<Value>> {
    cookies.remove_private(Cookie::named("token"));
    Ok(json!({"message": "Logout Successfully"}))
}

#[post("/api/register", format = "json", data = "<new_user>")]
async fn register(
    conn: DbConnection,
    new_user: Json<user_models::NewUser>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        UserRepository::create_account(c, new_user.into_inner())
            .map(|status| json!(status))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/api/token")]
async fn test_token(auth: JWTAuth) -> Value {
    json!(auth.user)
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
        .mount("/", routes![login, logout, register, test_token])
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
