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

//tests
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let pool = database::new_pool();

    if args.len() > 2 && args[1] == "load" {
        data_loader::load_data_from_file(&args[2], &pool.get().unwrap());
        return Ok(());
    }

    let bind_addr = match env::var("BIND_ADDRESS") {
        Ok(e) => e,
        Err(_) => String::from("127.0.0.1:8080"),
    };
    println!("Starting server on http://{}/", bind_addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(JsonConfig::default().limit(1_000_000 * 250))
            .data(web::PayloadConfig::new(1_000_000 * 250))
            .service(services::subscription::subscribe)
            .service(services::subscription::unsubscribe)
            .service(services::restaurant::restaurant)
            .service(services::restaurant::search_restaurants)
            .service(services::restaurant::restaurant_by_id)
            .service(services::admin::get_smiley_data)
            .service(services::admin::insert_smiley_data)
            .service(services::admin::update_smiley_data)
            .service(services::admin::delete_smiley_data)
    })
    .bind(bind_addr)?
    .run()
    .await
}
