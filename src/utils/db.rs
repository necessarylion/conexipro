use diesel::{
  r2d2::{ConnectionManager, Pool},
  MysqlConnection,
};
use r2d2::PooledConnection;

use super::get_env;
use crate::errors::handler::IError;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<MysqlConnection>>;

/// get database connection pool to use in main and add to web services
pub fn get_db_pool() -> DbPool {
  let url = get_env("DATABASE_URL").unwrap();
  let manager = ConnectionManager::<MysqlConnection>::new(url);
  Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}

///get database connection from pool
/// ```
/// let conn: &mut DbConn = get_db_conn(pool);
/// ```
pub fn get_db_conn(pool: &DbPool) -> Result<DbConn, IError> {
  pool
    .get()
    .map_err(|err| IError::ServerError(err.to_string()))
}

/// run database migration in PRODUCTION ONLY
/// required APP_ENV=production to run automatically
pub fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::mysql::Mysql>) {
  let app_env = get_env("APP_ENV").unwrap();
  if app_env == String::from("production") {
    log::info!("Started DB Migration");
    conn
      .run_pending_migrations(MIGRATIONS)
      .expect("Could not run migrations");
    log::info!("DB migrated");
  }
}
