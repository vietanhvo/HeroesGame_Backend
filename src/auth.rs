use dotenv::dotenv;
use std::env;

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use sha2::Sha256;

pub struct JWTAuth {
    pub token: String,
    pub user_email: String,
}

impl JWTAuth {
    fn from_token(token: &str) -> Option<JWTAuth> {
        dotenv().ok();
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(env::var("JWT_SECRET").unwrap().as_bytes()).unwrap();
        let user_email = token.verify_with_key(&key).unwrap();
        if user_email != "anhviet5121pc@gmail.com" {
            return None;
        }
        Some(JWTAuth {
            token: token.to_string(),
            user_email,
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token_cookie = req.cookies().get_private("token");
        if let Some(token_cookie) = token_cookie {
            if let Some(auth) = Self::from_token(token_cookie.value()) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
