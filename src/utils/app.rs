use diesel::{
  r2d2::{ConnectionManager, Pool},
  MysqlConnection,
};
use std::env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub struct AppState {
  pub db_pool: DbPool,
}

/**
 * get database connection pool
 * to use in main and add to web services
 */
pub fn get_db_pool() -> DbPool {
  let url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
  let manager = ConnectionManager::<MysqlConnection>::new(url);
  Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}
