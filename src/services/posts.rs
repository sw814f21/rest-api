use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use actix_web::{HttpResponse, Responder, get, post, delete, web};
use crate::database::models::Post;
use crate::database::models::Favorites;
use crate::database::models::User;


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


#[post("/add_favorite")]
pub async fn add_favorite(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, fav_info: web::Json<Favorites>) -> impl Responder{
    
    let conn = pool.get().unwrap();
    Favorites::add_favorite(fav_info.resturant_id, fav_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

#[delete("/remove_favorite")]
pub async fn remove_favorite(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, fav_info: web::Json<Favorites>) -> impl Responder{
    let conn = pool.get().unwrap();

    Favorites::remove_favorite(fav_info.resturant_id, fav_info.token_id.to_string(), &conn);

    HttpResponse::Ok()

}

#[get("/all_favorites")]
pub async fn all_favorites(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder{
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Favorites::list(&conn))
}

#[post("/new_user")]
pub async fn new_user(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, user_info: web::Json<User>) -> impl Responder{
    let conn = pool.get().unwrap();

    User::new_user(user_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

#[post("/update_notification")]
pub async fn update_notification(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, user_info: web::Json<User>) -> impl Responder{
    let conn = pool.get().unwrap();

    User::notification_change(user_info.token_id.to_string(), &conn);

    HttpResponse::Ok()
}

//cfg(test) tells compiler only to use this when running tests
#[cfg(test)]
mod tests{
    use actix_web::{
        App,
        web,
        test::{read_body, init_service, TestRequest}
    };
    use actix_web::http::Method;

    #[actix_rt::test]
    async fn test_manual_hello_string(){
        let mut app = init_service(App::new().route("/hey", web::get().to(super::manual_hello))).await;

        let result = TestRequest::get()
        .uri("/hey")
        .send_request(&mut app)
        .await;
        
        let data = read_body(result).await;
        match String::from_utf8(data.to_vec()){
            Ok(body) => assert_eq!(body, "Hey there!"),
            Err(_) => panic!("Couldnt parse response")
        }
    }

    #[actix_rt::test]
    async fn test_echo_hello_world(){
        let mut app = init_service(App::new().service(super::echo)).await;

        let result = TestRequest::get()
        .uri("/echo")
        .method(Method::POST)
        .set_payload("Hello, World!".as_bytes())
        .send_request(&mut app)
        .await;
        
        let data = read_body(result).await;
        match String::from_utf8(data.to_vec()){
            Ok(body) => assert_eq!(body, "Hello, World!"),
            Err(_) => panic!("Couldnt parse response")
        }
    }
}