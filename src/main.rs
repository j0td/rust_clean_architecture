#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod routes;
mod actions;
mod domain;
mod schema;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv().ok();

    let bind = "0.0.0.0:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routes::get_user)
            .service(routes::add_user)
    })
    .bind(&bind)?
    .run()
    .await
}

