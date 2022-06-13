use crate::schema::Monster;
use diesel::prelude::*;

pub struct MonsterRepository;

impl MonsterRepository {
    pub fn get_win_rate(conn: &MysqlConnection, monster_id: u64) -> QueryResult<u32> {
        Monster::table
            .filter(Monster::monster_id.eq(monster_id))
            .select(Monster::win_rate)
            .first::<u32>(conn)
    }
}

