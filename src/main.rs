use std::env;

use actix_web::{
  middleware::{self, Logger, TrailingSlash},
  App, HttpServer,
};
// use actix_web_lab::middleware::from_fn;
use conexipro::services::get_api_services;
use dotenv::dotenv;
use env_logger::Env;
use log::info;

extern crate conexipro;

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
      // .wrap(from_fn(my_mw))
      .wrap(Logger::default())
      .wrap(middleware::NormalizePath::new(TrailingSlash::default()))
      .service(get_api_services())
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await
}
