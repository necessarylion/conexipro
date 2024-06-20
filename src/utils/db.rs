use diesel::{
  r2d2::{ConnectionManager, Pool},
  MysqlConnection,
};
use r2d2::PooledConnection;

use crate::errors::handler::IError;

use super::get_env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<MysqlConnection>>;

/**
 * get database connection pool
 * to use in main and add to web services
 */
pub fn get_db_pool() -> DbPool {
  let url = get_env("DATABASE_URL").unwrap();
  let manager = ConnectionManager::<MysqlConnection>::new(url);
  Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}

pub fn get_db_conn(pool: &DbPool) -> Result<DbConn, IError> {
  pool
    .get()
    .map_err(|err| IError::ServerError(err.to_string()))
}
