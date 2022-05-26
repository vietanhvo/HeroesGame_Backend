use super::schema::User;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// Model for register users
#[derive(Insertable, Deserialize)]
#[table_name = "User"]
pub struct NewUser {
    pub first_name: String,
    pub surname: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub password: String,
}

// Model for login users
#[derive(Queryable, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}
