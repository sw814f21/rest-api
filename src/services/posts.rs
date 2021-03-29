use std::borrow::Borrow;

use crate::database::models::Favorites;
use crate::database::models::Post;
use crate::database::models::Restaurant;
use crate::database::models::User;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use array_tool;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::Deserialize;

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

#[get("/restaurant")]
pub async fn restaurant(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Restaurant::get_all_resturants(&conn))
}
/*
#[get("/restaurant/{id}")]
pub async fn restaurant_by_id(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let conn = pool.get().unwrap();


}*/

#[derive(Deserialize)]
pub struct Restaurantsearchinput {
    id: Option<i32>,
    name: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    location: Option<((f32, f32), (f32, f32))>,
}

use array_tool::vec::Intersect;
#[get("/restaurant/search")]
pub async fn search_restaurants(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    input: web::Query<Restaurantsearchinput>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let mut idsearch: Vec<Restaurant> = Vec::new();
    let mut namesearch: Vec<Restaurant> = Vec::new();
    let mut citysearch: Vec<Restaurant> = Vec::new();
    let mut zipsearch: Vec<Restaurant> = Vec::new();
    let mut locationsearch: Vec<Restaurant> = Vec::new();
    let mut queryoutput: Vec<Restaurant> = Vec::new();

    match input.id {
        None => {}
        Some(x) => {
            idsearch.append(vec![Restaurant::get_restaurant_by_id(x, &conn)].as_mut());
        }
    }
    match input.name.borrow() {
        None => {}
        Some(x) => {
            namesearch.append(Restaurant::search_by_name(x.to_string(), &conn).as_mut());
        }
    }
    match input.zip.borrow() {
        None => {}
        Some(x) => {
            zipsearch.append(Restaurant::search_by_zip(x.to_string(), &conn).as_mut());
        }
    }
    match input.city.borrow() {
        None => {}
        Some(x) => {
            citysearch.append(Restaurant::search_by_city(x.to_string(), &conn).as_mut());
        }
    }
    match input.location.borrow() {
        None => {}
        Some(x) => {
            let ne = x.0;
            let sw = x.1;
            locationsearch
                .append(Restaurant::search_by_lat_lng(ne.0, ne.1, sw.0, sw.1, &conn).as_mut());
        }
    }

    let results = vec![idsearch, namesearch, citysearch, zipsearch, locationsearch];
    for r in results {
        if queryoutput.is_empty() {
            queryoutput = r.to_vec();
        }
        if !r.is_empty() {
            queryoutput = queryoutput.intersect(r);
        }
    }
    HttpResponse::Ok().json(queryoutput)
}

#[post("/subscribe")]
pub async fn subscribe(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    fav_info: web::Json<Favorites>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    Favorites::add_favorite(fav_info.restaurant_id, fav_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

#[delete("/unsubscribe")]
pub async fn unsubscribe(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    fav_info: web::Json<Favorites>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    Favorites::remove_favorite(fav_info.restaurant_id, fav_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

#[get("/subscribes/{u_id}")]
pub async fn subscribes(
    web::Path(u_id): web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Favorites::user_favorites(u_id.to_string(), &conn))
}

#[post("/new_user")]
pub async fn new_user(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    user_info: web::Json<User>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    User::new_user(user_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

#[delete("/remove_user")]
pub async fn delete_user(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    user_info: web::Json<User>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    User::remove_user(user_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

#[post("/update_notification")]
pub async fn update_notification(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    user_info: web::Json<User>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    User::notification_change(user_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

//cfg(test) tells compiler only to use this when running tests
#[cfg(test)]
mod tests {
    use actix_web::http::Method;
    use actix_web::{
        test::{init_service, read_body, TestRequest},
        web, App,
    };

    #[actix_rt::test]
    async fn test_manual_hello_string() {
        let mut app =
            init_service(App::new().route("/hey", web::get().to(super::manual_hello))).await;

        let result = TestRequest::get().uri("/hey").send_request(&mut app).await;

        let data = read_body(result).await;
        match String::from_utf8(data.to_vec()) {
            Ok(body) => assert_eq!(body, "Hey there!"),
            Err(_) => panic!("Couldnt parse response"),
        }
    }

    #[actix_rt::test]
    async fn test_echo_hello_world() {
        let mut app = init_service(App::new().service(super::echo)).await;

        let result = TestRequest::get()
            .uri("/echo")
            .method(Method::POST)
            .set_payload("Hello, World!".as_bytes())
            .send_request(&mut app)
            .await;

        let data = read_body(result).await;
        match String::from_utf8(data.to_vec()) {
            Ok(body) => assert_eq!(body, "Hello, World!"),
            Err(_) => panic!("Couldnt parse response"),
        }
    }

    use crate::database;
    use crate::database::models::Post;

    #[actix_rt::test]
    async fn test_index_get_posts() {
        let x = database::establish_connection();

        let y = database::models::Post {
            id: 1,
            title: String::from("Header"),
            body: String::from("Body"),
            published: true,
        };

        let p_conn = x.get().unwrap();
        //database::run_migrations(&p_conn);
        Post::create(&p_conn, y).expect("Failed to create post");
        //database::run_migrations(&p_conn2);
        //let list = Post::list(&p_conn2);
        //assert_eq!(list[0].title, "XXXX Header XXXX");

        let mut app = init_service(App::new().data(x.clone()).service(super::hello)).await;

        let result = TestRequest::get().uri("/").send_request(&mut app).await;

        let data = read_body(result).await;
        match String::from_utf8(data.to_vec()) {
            Ok(body) => assert_eq!(
                body,
                "[{\"id\":1,\"title\":\"Header\",\"body\":\"Body\",\"published\":true}]"
            ),
            Err(_) => panic!("Couldnt parse response"),
        }
    }
}
