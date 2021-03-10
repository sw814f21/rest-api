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


use self::models::*;
use self::diesel::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = dotenv::var("DatabaseFile").unwrap();

    use crate::schema::posts::dsl::*;

    let db_conn = establish_connection(database_url);

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