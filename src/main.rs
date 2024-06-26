extern crate conexipro;
use actix_web::{
  middleware::{self, Logger, TrailingSlash},
  web::Data,
  App, HttpServer,
};
use conexipro::{
  middleware::{cors_middleware, rate_limit_middleware},
  routes,
  utils::db::{self, get_normal_db_connection},
  DbPool,
};
use dotenv::dotenv;
use env_logger::Env;
use std::{env, io::Result};

#[actix_web::main]
async fn main() -> Result<()> {
  // load env variables
  dotenv().ok();

  // enable logger
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  // get port from env
  let port: u16 = env::var("PORT").unwrap().parse().unwrap();

  // log server running port info
  log::info!("Server running at http://{}:{}", "127.0.0.1", port);

  let pool: DbPool = db::get_db_pool();

  let mut conn = get_normal_db_connection();
  db::run_db_migrations(&mut conn);

  HttpServer::new(move || {
    App::new()
      .wrap(cors_middleware::handler())
      .wrap(rate_limit_middleware::handler())
      .wrap(Logger::default())
      .wrap(middleware::NormalizePath::new(TrailingSlash::default()))
      .app_data(Data::new(pool.clone()))
      .service(routes::get_api_services())
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await
}
