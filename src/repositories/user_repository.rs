use crate::models::user_models::*;
use crate::schema::User;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;
use diesel::result::QueryResult;

pub struct UserRepository;

impl UserRepository {
    // Query
    pub fn find_by_id(c: &MysqlConnection, id: u64) -> QueryResult<UserInfo> {
        User::table.find(id).first::<UserInfo>(c)
    }

    pub fn find_by_email(c: &MysqlConnection, email: &str) -> QueryResult<UserInfo> {
        User::table
            .filter(User::email.eq(email))
            .first::<UserInfo>(c)
    }

    pub fn get_gold(c: &MysqlConnection, id: u64) -> QueryResult<u32> {
        User::table.find(id).select(User::gold).first::<u32>(c)
    }

    // Account
    pub fn create_account(conn: &MysqlConnection, mut new_user: NewUser) -> QueryResult<String> {
        // Create a salt
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .expect("Failed to hash password!")
            .to_string();
        new_user.password = password_hash;

        // Insert to database
        diesel::insert_into(User::table)
            .values(new_user)
            .execute(conn)?;

        Ok(String::from("Your account is created successfully!"))
    }

    pub fn verify_account(conn: &MysqlConnection, auth_user: AuthUser) -> Result<UserInfo, String> {
        // Query password by email in database
        let hashed_user_password: String = match User::table
            .filter(User::email.eq(&auth_user.email))
            .select(User::password)
            .first(conn)
        {
            Ok(hashed_password) => hashed_password,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        match Self::verify_password(&hashed_user_password, &auth_user.password) {
            Ok(compared_password) => {
                if compared_password {
                    // If password is correct, query user info
                    match Self::find_by_email(conn, &auth_user.email) {
                        Ok(user) => Ok(user),
                        Err(e) => Err(e.to_string()),
                    }
                } else {
                    Err(String::from("Your password is incorrect!"))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn verify_password(
        hashed_user_password: &str,
        user_password: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hashed_user_password)?;
        Ok(Argon2::default()
            .verify_password(user_password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    // Update
    pub fn receive_gold(
        conn: &MysqlConnection,
        user_id: u64,
        gold_increase: u32,
    ) -> QueryResult<u32> {
        let user_current_gold = Self::get_gold(conn, user_id)?;
        diesel::update(User::table.find(user_id))
            .set(User::gold.eq(user_current_gold + gold_increase))
            .execute(conn)?;
        Ok(user_current_gold + gold_increase)
    }

    pub fn pay_gold(
        conn: &MysqlConnection,
        user_id: u64,
        gold_decrease: u32,
    ) -> Result<u32, String> {
        let user_current_gold = match Self::get_gold(conn, user_id) {
            Ok(gold) => gold,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        if user_current_gold < gold_decrease {
            return Err(String::from("You don't have enough gold!"));
        }
        match diesel::update(User::table.find(user_id))
            .set(User::gold.eq(user_current_gold - gold_decrease))
            .execute(conn)
        {
            Ok(_) => Ok(user_current_gold - gold_decrease),
            Err(e) => Err(e.to_string()),
        }
    }
}
