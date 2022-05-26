use super::models::*;
use super::schema::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
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

    pub fn verify_account(conn: &MysqlConnection, auth_user: AuthUser) -> QueryResult<i32> {
        let hashed_user_password: String = User::table
            .filter(User::email.eq(auth_user.email))
            .select(User::password)
            .first(conn)
            .unwrap();
        Self::verify_password(&hashed_user_password);

        Self::last_id(conn)
    }

    fn last_id(conn: &MysqlConnection) -> QueryResult<i32> {
        User::table.select(User::id).order(User::id.desc()).first(conn)
    }

    fn verify_password(hashed_user_password: &str) -> bool {

    }
}
