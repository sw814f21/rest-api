#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

extern crate dotenv;

use dotenv::dotenv;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, JsonConfig}};

mod database;
use database::establish_connection;

pub mod schema;
pub mod models;
pub mod routes;

#[get("/")]
async fn hello(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder {

    let conn = pool.get().unwrap();

    use crate::schema::posts::dsl::*;

    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

    HttpResponse::Ok().json("Hello world!")
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

use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::SqliteConnection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = dotenv::var("DatabaseFile").unwrap();

    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB pool.");

    let conn = pool.get().unwrap();
    match embedded_migrations::run(&conn) {
        Ok(_v) => (),
        Err(_e) => panic!("Failed to run migrations")
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(JsonConfig::default().limit(4096))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(dotenv::var("Host").unwrap())?
    .run()
    .await
}