#[macro_use]
extern crate diesel;

pub mod database;

mod auth;
pub use auth::JWTAuth;

pub mod models;
pub mod repositories;
pub mod schema;
