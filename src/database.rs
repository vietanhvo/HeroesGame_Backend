use rocket::Build;
use rocket_sync_db_pools::database;

embed_migrations!();

#[database("mysql_db")]
pub struct DbConnection(diesel::MysqlConnection);

pub async fn run_db_migrations(
    rocket: rocket::Rocket<Build>,
) -> Result<rocket::Rocket<Build>, rocket::Rocket<Build>> {
    DbConnection::get_one(&rocket)
        .await
        .expect("Failed to retrieve database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}
