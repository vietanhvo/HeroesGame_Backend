use super::database::DbConnection;
use dotenv::dotenv;
use std::env;

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use sha2::Sha256;

use crate::{models::UserInfo, repositories::UserRepository};

pub struct JWTAuth {
    pub token: String,
    pub user: UserInfo,
}

impl JWTAuth {
    async fn from_token(conn: DbConnection, token: &str) -> Option<JWTAuth> {
        dotenv().ok();
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(env::var("JWT_SECRET").unwrap().as_bytes()).unwrap();
        // Get user email from JWT token
        let user_email: String = match token.verify_with_key(&key) {
            Ok(user_email) => user_email,
            Err(_) => return None,
        };

        Self::from_database(conn, user_email, token).await
    }

    async fn from_database(conn: DbConnection, user_email: String, token: &str) -> Option<JWTAuth> {
        // Query user's information by email
        let user = conn
            .run(move |c| {
                UserRepository::find_by_email(c, &user_email)
                    .map(|user_info| user_info)
                    .map_err(|e| e)
            })
            .await;

        if let Ok(user) = user {
            return Some(JWTAuth {
                token: token.to_string(),
                user,
            });
        }

        None
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token_cookie = req.cookies().get_private("token");
        if let Some(token_cookie) = token_cookie {
            if let Outcome::Success(conn) = DbConnection::from_request(req).await {
                if let Some(auth) = Self::from_token(conn, token_cookie.value()).await {
                    return Outcome::Success(auth);
                }
            };
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
