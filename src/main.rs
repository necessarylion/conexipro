extern crate conexipro;

use std::env;

use actix_web::{
  middleware::{self, Logger, TrailingSlash},
  App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use log::info;

use conexipro::{
  middleware::{cors::get_cors_config, rate_limit::get_rate_limit_config},
  services::get_api_services,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // load env variables
  dotenv().ok();

  // enable logger
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  // get port from env
  let port: u16 = env::var("PORT").unwrap().parse().unwrap();

  // log server running port info
  info!("Server running at http://{}:{}", "127.0.0.1", port);

  HttpServer::new(|| {
    App::new()
      .wrap(get_cors_config())
      .wrap(get_rate_limit_config())
      .wrap(Logger::default())
      .wrap(middleware::NormalizePath::new(TrailingSlash::default()))
      .service(get_api_services())
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await
}
