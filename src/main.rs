#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use dotenv::dotenv;
use actix_web::{App, HttpServer, web::{self, JsonConfig}};

pub mod database;
pub mod services;

embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // TOOO: Grab connection pool from database.rs's function instead of here. Awaiting Thorulf's testing setup
    let database_url = dotenv::var("DatabaseFile").unwrap();

    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB pool.");

    // TODO: Move migrations to database.rs
    let conn = pool.get().unwrap();
    match embedded_migrations::run(&conn) {
        Ok(_v) => (),
        Err(_e) => panic!("Failed to run migrations")
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(JsonConfig::default().limit(4096))
            .service(services::posts::hello)
            .service(services::posts::echo)
            .route("/hey", web::get().to(services::posts::manual_hello))
    })
    .bind(dotenv::var("Host").unwrap())?
    .run()
    .await
}