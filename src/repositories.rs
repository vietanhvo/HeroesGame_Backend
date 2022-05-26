use super::models::*;
use super::schema::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;
use diesel::result::QueryResult;

pub struct UserRepository;

impl UserRepository {
    pub fn create_account(conn: &MysqlConnection, mut new_user: NewUser) -> QueryResult<i32> {
        // Create a salt
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        new_user.password = password_hash;

        diesel::insert_into(User::table)
            .values(new_user)
            .execute(conn)?;

        Self::last_id(conn)
    }

    fn last_id(c: &MysqlConnection) -> QueryResult<i32> {
        User::table.select(User::id).order(User::id.desc()).first(c)
    }
}
