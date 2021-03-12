use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use actix_web::{HttpResponse, Responder, get, post, web};
use crate::database::models::Post;

#[get("/")]
pub async fn hello(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Post::list(&conn))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}