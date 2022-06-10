use dotenv::dotenv;
use std::env;

use crate::auth::JWTAuth;
use crate::database::DbConnection;
use crate::models::user_models::*;
use crate::repositories::UserRepository;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use sha2::Sha256;

#[post("/login", format = "json", data = "<user_auth>")]
pub async fn login(
    conn: DbConnection,
    user_auth: Json<AuthUser>,
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
            // for cookies can be stored in web browser on localhost
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

#[get("/logout", format = "json")]
pub async fn logout(
    _auth: JWTAuth,
    cookies: &CookieJar<'_>,
) -> Result<Value, status::Custom<Value>> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .secure(true)
        .same_site(rocket::http::SameSite::None)
        .http_only(true);

    cookies.remove_private(cookie.finish());
    Ok(json!({"message": "Logout Successfully"}))
}

#[post("/register", format = "json", data = "<new_user>")]
pub async fn register(
    conn: DbConnection,
    new_user: Json<NewUser>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        UserRepository::create_account(c, new_user.into_inner())
            .map(|status| json!(status))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/gold", format = "json")]
pub async fn get_gold(auth: JWTAuth, conn: DbConnection) -> Result<Value, status::Custom<Value>> {
    let user_id = auth.user.user_id;
    conn.run(move |c| {
        UserRepository::get_gold(c, user_id)
            .map(|gold| json!(gold))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/token")]
pub async fn test_token(auth: JWTAuth) -> Value {
    json!(auth.user)
}
