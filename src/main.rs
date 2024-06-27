extern crate conexipro;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use conexipro::{
  middleware::{cors_middleware, rate_limit_middleware},
  routes,
  swagger::{crate_swagger_file, ApiDoc},
  utils::db::{self, get_normal_db_connection},
  DbPool,
};
use dotenv::dotenv;
use env_logger::Env;
use std::{env, io::Result};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<()> {
  // load env variables
  dotenv().ok();

  crate_swagger_file();

  // enable logger
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  // get port from env
  let port: u16 = env::var("PORT").unwrap().parse().unwrap();

  // log server running port info
  log::info!("Server running at http://{}:{}", "127.0.0.1", port);

  let pool: DbPool = db::get_db_pool().await;

  let mut conn = get_normal_db_connection();
  db::run_db_migrations(&mut conn);

  HttpServer::new(move || {
    App::new()
      .wrap(cors_middleware::handler())
      .wrap(rate_limit_middleware::handler())
      .wrap(Logger::default())
      .service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
      )
      .app_data(Data::new(pool.clone()))
      .service(routes::get_api_services())
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await
}
