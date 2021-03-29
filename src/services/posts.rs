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

#[get("/restaurant/{id}")]
pub async fn restaurant_by_id(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Restaurant::get_restaurant_by_id(id, &conn))
}

#[derive(Deserialize)]
pub struct LatLngQuery {
    northeast: String,
    southwest: String,
}

#[derive(Deserialize)]
pub struct NameQuery {
    name: String,
}

#[get("/restaurants/search")]
pub async fn restaurants_search(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    query: web::Query<LatLngQuery>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let mut northeast = query.northeast.split(",");
    let mut southwest = query.southwest.split(",");
    let nelat = northeast.next().unwrap().parse::<f32>().unwrap();
    let nelng = northeast.next().unwrap().parse::<f32>().unwrap();
    let swlat = southwest.next().unwrap().parse::<f32>().unwrap();
    let swlng = southwest.next().unwrap().parse::<f32>().unwrap();
    HttpResponse::Ok().json(Restaurant::search_by_lat_lng(
        nelat, nelng, swlat, swlng, &conn,
    ))
}

#[get("/restaurant/search")]
pub async fn restaurant_search(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    query: web::Query<NameQuery>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    HttpResponse::Ok().json(Restaurant::search_by_name(query.name.to_string(), &conn))
}

use array_tool::vec::Intersect;
use std::collections::HashMap;
#[get("/search/{request}")]
pub async fn search_restaurants(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    web::Path(request): web::Path<String>,
) -> impl Responder {
    /*
    Request format (
    "=" Assign a Field name to a value,
    "&" Field seperator,
    "," value seperator) */
    /*
    /search/This=is,a,test&For=shitty&implementation=:ok_hand:
    This = (is, a, test)
    For = shitty
    implementation = :ok_hand:
     */
    let conn = pool.get().unwrap();
    let query = request.split("&");
    let mut kvpair = HashMap::new();

    for q in query {
        let mut pair = q.split("=");
        let varname = pair.next().unwrap().to_lowercase();
        let param = match pair.next() {
            None => "".split(""),
            Some(x) => x.split(","),
        };
        kvpair.insert(varname, param.to_owned());
    }
    let mut queryoutput: Vec<Restaurant> = Vec::new();
    let mut namesearch: Vec<Restaurant> = Vec::new();
    let mut citysearch: Vec<Restaurant> = Vec::new();
    let mut zipsearch: Vec<Restaurant> = Vec::new();
    let mut locationsearch: Vec<Restaurant> = Vec::new();
    /*for p in kvpair {
        outputtest = outputtest + p.0 + ":";
        for q in p.1 {
            outputtest = outputtest + q + " ";
        }
        outputtest += "||";
    }*/
    for key in kvpair.keys() {
        if key.eq(&"name") {
            namesearch.append(
                Restaurant::search_by_name(
                    kvpair
                        .get(key)
                        .unwrap()
                        .to_owned()
                        .fold(String::from(""), |acc, x| acc + &(x.to_string() + " "))
                        .trim_end()
                        .to_string(),
                    &conn,
                )
                .as_mut(),
            );
        }
        if key.eq(&"zipcode") {
            zipsearch.append(
                Restaurant::search_by_zip(
                    kvpair
                        .get(key)
                        .unwrap()
                        .to_owned()
                        .fold(String::from(""), |acc, x| acc + x)
                        .trim_end()
                        .to_string(),
                    &conn,
                )
                .as_mut(),
            );
        }
        if key.eq(&"city") {
            citysearch.append(
                Restaurant::search_by_city(
                    kvpair
                        .get(key)
                        .unwrap()
                        .to_owned()
                        .fold(String::from(""), |acc, x| acc + &(x.to_string() + " "))
                        .trim_end()
                        .to_string(),
                    &conn,
                )
                .as_mut(),
            );
        }
        if key.eq("northeast") && kvpair.get("southwest").is_some() {
            let mut northeast = kvpair.get(key).unwrap().to_owned();
            let mut southwest = kvpair.get("southwest").unwrap().to_owned();
            let nelat = northeast.next().unwrap().parse::<f32>().unwrap();
            let nelng = northeast.next().unwrap().parse::<f32>().unwrap();
            let swlat = southwest.next().unwrap().parse::<f32>().unwrap();
            let swlng = southwest.next().unwrap().parse::<f32>().unwrap();
            locationsearch
                .append(Restaurant::search_by_lat_lng(nelat, nelng, swlat, swlng, &conn).as_mut());
        }
    }
    let results = vec![namesearch, citysearch, zipsearch, locationsearch];
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
