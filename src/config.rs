use rocket_db_pools::diesel::MysqlPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("diesel_mysql")]
pub struct Db(MysqlPool);
