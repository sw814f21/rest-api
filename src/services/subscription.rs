
use actix_web::{delete, post, web, HttpResponse, Responder};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use crate::database::models::Subscription;

#[post("/subscribe")]
pub async fn subscribe(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    fav_info: web::Json<Subscription>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    Subscription::subscribe(fav_info.restaurant_id, &fav_info.token, &conn);

    HttpResponse::Ok()
}

#[delete("/unsubscribe")]
pub async fn unsubscribe(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    fav_info: web::Json<Subscription>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    Subscription::unsubscribe(fav_info.restaurant_id, &fav_info.token, &conn);

    HttpResponse::Ok()
}