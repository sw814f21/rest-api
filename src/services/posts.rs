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

    use crate::database;
    use crate::database::models::Post;

    #[actix_rt::test]
    async fn test_index_get_posts(){
        let x = database::establish_connection();

        let y = database::models::Post{
            id: 1,
            title: String::from("Header"),
            body: String::from("Body"),
            published: true
        };

        let p_conn = x.get().unwrap();
        //database::run_migrations(&p_conn);
        Post::create(&p_conn, y).expect("Failed to create post");
        //database::run_migrations(&p_conn2);
        //let list = Post::list(&p_conn2);
        //assert_eq!(list[0].title, "XXXX Header XXXX");

        let mut app = init_service(App::new().data(x.clone()).service(super::hello)).await;
        
        let result = TestRequest::get()
        .uri("/")
        .send_request(&mut app)
        .await;
        


        let data = read_body(result).await;
        match String::from_utf8(data.to_vec()){
            Ok(body) => assert_eq!(body, "[{\"id\":1,\"title\":\"Header\",\"body\":\"Body\",\"published\":true}]"),
            Err(_) => panic!("Couldnt parse response")
        }

    }
}
