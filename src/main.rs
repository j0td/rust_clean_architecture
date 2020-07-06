#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;

mod routes;
mod actions;
mod domain;
mod interface;
mod schema;

// topレベルでやらないといけないらしいので、ここにroutingを書いていく


#[actix_rt::main]
async fn main() {
    env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv().ok();

    routes::initialize().await.expect("err");
}
