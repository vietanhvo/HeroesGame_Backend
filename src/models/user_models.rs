use crate::schema::User;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserInfo {
    pub user_id: u64,
    pub first_name: String,
    pub surname: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    // Don't send this field in the response
    #[serde(skip_serializing)]
    pub password: String,
    pub gold: u32,
}

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
    pub remember: bool,
}
