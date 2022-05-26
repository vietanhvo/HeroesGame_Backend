use super::models::*;
use super::schema::*;
use diesel::prelude::*;
use diesel::result::QueryResult;

pub struct UserRepository;

impl UserRepository {
    pub fn create_account(conn: &MysqlConnection, new_user: NewUser) -> QueryResult<i32> {
        diesel::insert_into(User::table)
            .values(new_user)
            .execute(conn)?;

        Self::last_id(conn)
    }

    fn last_id(c: &MysqlConnection) -> QueryResult<i32> {
        User::table.select(User::id).order(User::id.desc()).first(c)
    }
}
