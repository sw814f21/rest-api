use actix_web::{delete, post, web, HttpResponse, Responder};

use crate::database;
use crate::database::schema::subscription;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "subscription"]
pub struct SubscriptionRequest {
    pub restaurant_id: i32,
    pub token: String,
}

#[post("/subscribe")]
pub async fn subscribe(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    fav_info: web::Json<SubscriptionRequest>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let request: SubscriptionRequest = SubscriptionRequest {
        restaurant_id: fav_info.restaurant_id,
        token: fav_info.token.to_string(),
    };
    database::models::Subscription::subscribe(request, &conn);

    HttpResponse::Ok()
}

#[delete("/unsubscribe")]
pub async fn unsubscribe(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    fav_info: web::Json<SubscriptionRequest>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let request: SubscriptionRequest = SubscriptionRequest {
        restaurant_id: fav_info.restaurant_id,
        token: fav_info.token.to_string(),
    };

    database::models::Subscription::unsubscribe(request, &conn);

    HttpResponse::Ok()
}
