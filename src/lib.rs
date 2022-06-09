#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;

pub mod database;
pub mod auth;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schema;
