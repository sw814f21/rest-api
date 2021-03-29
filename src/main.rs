#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::utils::data_loader;
use actix_web::{
    web::{self, JsonConfig},
    App, HttpServer,
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod database;
pub mod services;
pub mod utils;

embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && args[1] == "load" {
        data_loader::load_data(&args[2]);
        return Ok(());
    }

    dotenv().ok();

    // TOOO: Grab connection pool from database.rs's function instead of here. Awaiting Thorulf's testing setup
    let database_url = dotenv::var("DatabaseFile").unwrap();

    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    // TODO: Move migrations to database.rs
    let conn = pool.get().unwrap();
    match embedded_migrations::run(&conn) {
        Ok(_v) => (),
        Err(_e) => panic!("Failed to run migrations"),
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(JsonConfig::default().limit(4096))
            .service(services::posts::hello)
            .service(services::posts::echo)
            .service(services::posts::subscribe)
            .service(services::posts::unsubscribe)
            .service(services::posts::subscribes)
            .service(services::posts::new_user)
            .service(services::posts::restaurant)
            .service(services::posts::restaurant_by_id)
            .service(services::posts::restaurants_search)
            .service(services::posts::restaurant_search)
            .route("/hey", web::get().to(services::posts::manual_hello))
    })
    .bind(dotenv::var("Host").unwrap())?
    .run()
    .await
}
