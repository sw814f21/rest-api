#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

extern crate dotenv;

use dotenv::dotenv;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod database;
use database::establish_connection;

pub mod schema;
pub mod models;

#[get("/")]
async fn hello() -> impl Responder {

    let database_url = dotenv::var("DatabaseFile").unwrap();

    let db_conn = establish_connection(database_url);

    match embedded_migrations::run(&db_conn) {
        Ok(_v) => (),
        Err(_e) => panic!("Failed to run migrations")
    }

    use crate::schema::posts::dsl::*;

    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&db_conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


use self::models::*;
use self::diesel::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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