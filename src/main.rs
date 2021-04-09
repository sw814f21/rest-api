#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::utils::data_loader;
use actix_web::{
    web::{self, JsonConfig},
    App, HttpServer,
};
use dotenv::dotenv;
use std::env;

pub mod database;

//Services
mod services;

//Utils
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && args[1] == "load" {
        data_loader::load_data(&args[2]);
        return Ok(());
    }

    let pool = database::new_pool();
    let bind_addr = match env::var("BIND_ADDRESS") {
        Ok(e) => e,
        Err(_) => String::from("127.0.0.1:8080"),
    };

    println!("Starting server on http://{}/", bind_addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(JsonConfig::default().limit(4096))
            .service(services::example::echo)
            .service(services::subscription::subscribe)
            .service(services::subscription::unsubscribe)
            .service(services::restaurant::restaurant)
            .service(services::restaurant::search_restaurants)
            .service(services::restaurant::restaurant_by_id)
            .route("/hey", web::get().to(services::example::manual_hello))
    })
    .bind(bind_addr)?
    .run()
    .await
}
