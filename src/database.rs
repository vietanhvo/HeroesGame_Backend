use rocket_sync_db_pools::database;

#[database("mysql_db")]
pub struct DbConnection(diesel::MysqlConnection);
