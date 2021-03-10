#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;

use dotenv::dotenv;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod database;
use database::establish_connection;

pub mod schema;
pub mod models;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = dotenv::var("DatabaseFile").unwrap();
    let db_conn = establish_connection(database_url);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(dotenv::var("Host").unwrap())?
    .run()
    .await
}