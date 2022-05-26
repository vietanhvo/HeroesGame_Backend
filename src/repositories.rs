use super::models::*;
use super::schema::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;
use diesel::result::{Error, QueryResult};

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_id(c: &MysqlConnection, id: i32) -> QueryResult<UserInfo> {
        User::table.find(id).get_result::<UserInfo>(c)
    }

    pub fn find_by_email(c: &MysqlConnection, email: &str) -> QueryResult<UserInfo> {
        User::table
            .filter(User::email.eq(email))
            .get_result::<UserInfo>(c)
    }

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

    pub fn verify_account(conn: &MysqlConnection, auth_user: AuthUser) -> QueryResult<UserInfo> {
        // Query password by email in database
        let hashed_user_password: String = User::table
            .filter(User::email.eq(&auth_user.email))
            .select(User::password)
            .first(conn)?;

        let compared_password = Self::verify_password(&hashed_user_password, &auth_user.password)
            .expect("Password is not correct!");
        if compared_password {
            Self::find_by_email(conn, &auth_user.email)
        } else {
            Err(Error::NotFound)
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
}
