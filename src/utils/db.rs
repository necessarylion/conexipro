use super::get_env;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncMysqlConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub type DbPool = Pool<AsyncMysqlConnection>;

/// get database connection pool to use in main and add to web services
pub fn get_db_pool() -> DbPool {
  let url = get_env("DATABASE_URL").unwrap();
  let config = AsyncDieselConnectionManager::<diesel_async::AsyncMysqlConnection>::new(url);
  Pool::builder(config).build().unwrap()
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
