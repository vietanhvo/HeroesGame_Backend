use rocket::serde::json::{json, Value};

#[catch(401)]
pub fn unauthorized(_req: &rocket::Request) -> Value {
    json!("Unauthorized")
}

#[catch(404)]
pub fn not_found(_req: &rocket::Request) -> Value {
    json!("Not found")
}
