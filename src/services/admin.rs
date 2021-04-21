use crate::database::models::Restaurant;
use crate::utils::data_loader;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

#[get("/admin/ids")]
pub async fn get_ids(
    req: HttpRequest,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    if is_localhost(req) {
        HttpResponse::Ok().json(Restaurant::get_restaurant_references(&pool.get().unwrap()))
    } else {
        HttpResponse::build(StatusCode::from_u16(404).expect("Failed to create status code"))
            .finish()
    }
}

#[post("/admin/insert")]
pub async fn load_data(
    req: HttpRequest,
    req_body: String,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    if is_localhost(req) {
        data_loader::load_data(&req_body, &pool.get().unwrap());

        HttpResponse::Ok().body(req_body)
    } else {
        HttpResponse::build(StatusCode::from_u16(404).expect("Failed to create status code"))
            .finish()
    }
}

#[delete("/admin/load")]
pub async fn delete_smiley_entries(
    req: HttpRequest,
    req_body: String,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    if is_localhost(req) {
        data_loader::delete_smiley_records(&req_body, &pool.get().unwrap());

        HttpResponse::Ok().finish()
    } else {
        HttpResponse::build(StatusCode::from_u16(404).expect("Failed to create status code"))
            .finish()
    }
}

fn is_localhost(req: HttpRequest) -> bool {
    let conn_info = req.connection_info();
    let address = match conn_info.remote_addr() {
        Some(address) => address,
        None => "",
    };
    address.contains("127.0.0.1")
}

#[cfg(test)]
mod tests {
    use crate::database;
    use crate::database::models::Restaurant;
    use actix_web::http::Method;
    use actix_web::{
        test::{init_service, TestRequest},
        App,
    };
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[actix_rt::test]
    async fn test_load_single_entry() {
        let db_pool = database::new_pool();
        println!("wow");

        let mut app =
            init_service(App::new().data(db_pool.clone()).service(super::load_data)).await;
        println!("wow");

        //Send a request with a single restaurant in the body in json format
        TestRequest::get()
            .uri("/admin/insert")
            .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))
            .method(Method::POST)
            .set_payload("[    {        \"cvrnr\": \"27560946\",        \"pnr\": \"1012180825\",        \"region\": null,        \"industry_code\": \"472400\",        \"industry_text\": \"Detailhandel med brød, konditori- og sukkervarer\",        \"start_date\": \"2005-06-01T00:00:00Z\",        \"smiley_reports\": [            {                \"report_id\": \"Virk1873390\",                \"smiley\": 1,                \"date\": \"2021-03-22T00:00:00Z\"            },            {                \"report_id\": \"Virk1834536\",                \"smiley\": 1,                \"date\": \"2020-11-11T00:00:00Z\"            },            {                \"report_id\": \"Virk1789610\",                \"smiley\": 1,                \"date\": \"2020-08-04T00:00:00Z\"            },            {                \"report_id\": \"Virk1771244\",                \"smiley\": 2,                \"date\": \"2020-06-10T00:00:00Z\"            }        ],        \"city\": \"Charlottenlund\",        \"elite_smiley\": \"0\",        \"geo_lat\": 55.762464,        \"geo_lng\": 12.585801,        \"franchise_name\": null,        \"niche_industry\": \"Bagere og bagerafdelinger\",        \"url\": \"http://www.findsmiley.dk/da-DK/Searching/DetailsView.htm?virk=757164\",        \"address\": \"Ordrup Jagtvej 42B, st\",        \"name\": \"Patricks Bake Shop - Ordrup ApS\",        \"name_seq_nr\": \"757164\",        \"zip_code\": \"2920\",        \"ad_protection\": \"0\",        \"company_type\": \"Detail\"    }]".as_bytes())
            .send_request(&mut app)
            .await;

        println!("wow");

        let restaurant_vec =
            Restaurant::get_all_resturants(&db_pool.get().expect("Cant get database connection"));

        println!("wow");

        assert_eq!(restaurant_vec.len(), 1);
    }

    #[actix_rt::test]
    async fn test_remote_connection() {
        let db_pool = database::new_pool();

        let mut app =
            init_service(App::new().data(db_pool.clone()).service(super::load_data)).await;

        //Send a request with a single restaurant in the body in json format
        TestRequest::get()
            .uri("/admin/insert")
            .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(54, 231, 65, 23)), 8080))
            .method(Method::POST)
            .set_payload("[    {        \"cvrnr\": \"27560946\",        \"pnr\": \"1012180825\",        \"region\": null,        \"industry_code\": \"472400\",        \"industry_text\": \"Detailhandel med brød, konditori- og sukkervarer\",        \"start_date\": \"2005-06-01T00:00:00Z\",        \"smiley_reports\": [            {                \"report_id\": \"Virk1873390\",                \"smiley\": 1,                \"date\": \"2021-03-22T00:00:00Z\"            },            {                \"report_id\": \"Virk1834536\",                \"smiley\": 1,                \"date\": \"2020-11-11T00:00:00Z\"            },            {                \"report_id\": \"Virk1789610\",                \"smiley\": 1,                \"date\": \"2020-08-04T00:00:00Z\"            },            {                \"report_id\": \"Virk1771244\",                \"smiley\": 2,                \"date\": \"2020-06-10T00:00:00Z\"            }        ],        \"city\": \"Charlottenlund\",        \"elite_smiley\": \"0\",        \"geo_lat\": 55.762464,        \"geo_lng\": 12.585801,        \"franchise_name\": null,        \"niche_industry\": \"Bagere og bagerafdelinger\",        \"url\": \"http://www.findsmiley.dk/da-DK/Searching/DetailsView.htm?virk=757164\",        \"address\": \"Ordrup Jagtvej 42B, st\",        \"name\": \"Patricks Bake Shop - Ordrup ApS\",        \"name_seq_nr\": \"757164\",        \"zip_code\": \"2920\",        \"ad_protection\": \"0\",        \"company_type\": \"Detail\"    }]".as_bytes())
            .send_request(&mut app)
            .await;

        let restaurant_vec =
            Restaurant::get_all_resturants(&db_pool.get().expect("Cant get database connection"));

        assert_eq!(restaurant_vec.len(), 0);
    }

    #[actix_rt::test]
    async fn test_extract_id() {
        let db_pool = database::new_pool();

        let mut app =
            init_service(App::new().data(db_pool.clone()).service(super::load_data)).await;

        let mut restaurant_vec = Restaurant::get_restaurant_references(
            &db_pool.get().expect("Cant get database connection"),
        );
        assert_eq!(restaurant_vec.len(), 0);

        //Send a request with a single restaurant in the body in json format
        TestRequest::get()
            .uri("/admin/insert")
            .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))
            .method(Method::POST)
            .set_payload("[{\"cvrnr\": \"27560946\",\"pnr\": \"1012180825\",\"region\": null,\"industry_code\": \"472400\",\"industry_text\": \"Detailhandel med brød, konditori- og sukkervarer\",\"start_date\": \"2005-06-01T00:00:00Z\",        \"smiley_reports\": [            {                \"report_id\": \"Virk1873390\",                \"smiley\": 1,                \"date\": \"2021-03-22T00:00:00Z\"            },            {                \"report_id\": \"Virk1834536\",                \"smiley\": 1,                \"date\": \"2020-11-11T00:00:00Z\"            },            {                \"report_id\": \"Virk1789610\",                \"smiley\": 1,                \"date\": \"2020-08-04T00:00:00Z\"            },            {                \"report_id\": \"Virk1771244\",                \"smiley\": 2,                \"date\": \"2020-06-10T00:00:00Z\"            }        ],        \"city\": \"Charlottenlund\",        \"elite_smiley\": \"0\",        \"geo_lat\": 55.762464,        \"geo_lng\": 12.585801,        \"franchise_name\": null,        \"niche_industry\": \"Bagere og bagerafdelinger\",        \"url\": \"http://www.findsmiley.dk/da-DK/Searching/DetailsView.htm?virk=757164\",        \"address\": \"Ordrup Jagtvej 42B, st\",        \"name\": \"Patricks Bake Shop - Ordrup ApS\",        \"name_seq_nr\": \"757164\",        \"zip_code\": \"2920\",        \"ad_protection\": \"0\",        \"company_type\": \"Detail\"    }]".as_bytes())
            .send_request(&mut app)
            .await;

        restaurant_vec = Restaurant::get_restaurant_references(
            &db_pool.get().expect("Cant get database connection"),
        );

        assert_eq!(restaurant_vec.len(), 1);
        assert_eq!(restaurant_vec[0], 757164)
    }

    #[actix_rt::test]
    async fn test_extract_ids() {
        let db_pool = database::new_pool();

        let mut app =
            init_service(App::new().data(db_pool.clone()).service(super::load_data)).await;

        let mut restaurant_vec = Restaurant::get_restaurant_references(
            &db_pool.get().expect("Cant get database connection"),
        );
        assert_eq!(restaurant_vec.len(), 0);

        //Send a request with two restaurants in the body in json format
        TestRequest::get()
            .uri("/admin/insert")
            .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))
            .method(Method::POST)
            .set_payload("[{\"cvrnr\":\"27560946\",\"pnr\":\"1012180825\",\"region\":null,\"industry_code\":\"472400\",\"industry_text\":\"aba\",\"start_date\":\"2005-06-01T00:00:00Z\",\"smiley_reports\":[{\"report_id\":\"Virk1873390\",\"smiley\":1,\"date\":\"2021-03-22T00:00:00Z\"},{\"report_id\":\"Virk1834536\",\"smiley\":1,\"date\":\"2020-11-11T00:00:00Z\"},{\"report_id\":\"Virk1789610\",\"smiley\":1,\"date\":\"2020-08-04T00:00:00Z\"},{\"report_id\":\"Virk1771244\",\"smiley\":2,\"date\":\"2020-06-10T00:00:00Z\"}],\"city\":\"aba\",\"elite_smiley\":\"0\",\"geo_lat\":55.762464,\"geo_lng\":12.585801,\"franchise_name\":null,\"niche_industry\":\"aba\",\"url\":\"aba\",\"address\":\"aba\",\"name\":\"abba\",\"name_seq_nr\":\"1\",\"zip_code\":\"2300\",\"ad_protection\":\"0\",\"company_type\":\"Detail\"},{\"cvrnr\":\"27560946\",\"pnr\":\"1012180825\",\"region\":null,\"industry_code\":\"472400\",\"industry_text\":\"aba\",\"start_date\":\"2005-06-01T00:00:00Z\",\"smiley_reports\":[{\"report_id\":\"Virk1873390\",\"smiley\":1,\"date\":\"2021-03-22T00:00:00Z\"},{\"report_id\":\"Virk1834536\",\"smiley\":1,\"date\":\"2020-11-11T00:00:00Z\"},{\"report_id\":\"Virk1789610\",\"smiley\":1,\"date\":\"2020-08-04T00:00:00Z\"},{\"report_id\":\"Virk1771244\",\"smiley\":2,\"date\":\"2020-06-10T00:00:00Z\"}],\"city\":\"aba\",\"elite_smiley\":\"0\",\"geo_lat\":55.762464,\"geo_lng\":12.585801,\"franchise_name\":null,\"niche_industry\":\"aba\",\"url\":\"aba\",\"address\":\"aba\",\"name\":\"abba\",\"name_seq_nr\":\"2\",\"zip_code\":\"2300\",\"ad_protection\":\"0\",\"company_type\":\"Detail\"}]".as_bytes())
            .send_request(&mut app)
            .await;

        restaurant_vec = Restaurant::get_restaurant_references(
            &db_pool.get().expect("Cant get database connection"),
        );

        assert_eq!(restaurant_vec.len(), 2);
        assert!(restaurant_vec.contains(&1));
        assert!(restaurant_vec.contains(&2));
    }
}
