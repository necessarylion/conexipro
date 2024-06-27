use super::get_env;
use crate::IError;
use bb8::PooledConnection;
use diesel::{Connection, MysqlConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{pooled_connection::bb8::Pool, AsyncMysqlConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;

/// types
pub type DbPool = Pool<AsyncMysqlConnection>;
pub type DbConn<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncMysqlConnection>>;

/// get database connection pool to use in main and add to web services
pub async fn get_db_pool() -> DbPool {
  let url = get_env("DATABASE_URL").unwrap();
  let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(url);
  Pool::builder()
    .max_size(20)
    .min_idle(5)
    .build(config)
    .await
    .unwrap()
}

/// get database connection from pool
pub async fn get_db_conn(pool: &Arc<DbPool>) -> Result<DbConn, IError> {
  pool
    .get()
    .await
    .map_err(|err| IError::ServerError(err.to_string()))
}

/// get normal db connection for migration
pub fn get_normal_db_connection() -> MysqlConnection {
  let url = get_env("DATABASE_URL").unwrap();
  MysqlConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

/// run database migration in PRODUCTION ONLY
/// required APP_ENV=production to run automatically
pub fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::mysql::Mysql>) {
  const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
  let app_env = get_env("APP_ENV").unwrap();
  if app_env == String::from("production") {
    log::info!("Started DB Migration");
    conn
      .run_pending_migrations(MIGRATIONS)
      .expect("Could not run migrations");
    log::info!("DB migrated");
  }
}
