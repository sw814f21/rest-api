use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use actix_web::{post, web, HttpResponse, Responder, HttpRequest};
use actix_web::http::StatusCode;
use crate::utils::data_loader;

#[post("/admin/insert")]
pub async fn load_data(req: HttpRequest ,req_body: String, pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder {
    
    let conn_info = req.connection_info();
    let address = match conn_info.remote_addr() {
        Some(address) => address,
        None => panic!("Couldnt get remote address")
    };
    
    if address.contains("127.0.0.1") {
        data_loader::load_data(&req_body, &pool.get().unwrap());
    
        HttpResponse::Ok().body(req_body)
    } else {
        HttpResponse::build(StatusCode::from_u16(404).expect("Failed to create status code")).finish()
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::Method;
    use actix_web::{
        test::{init_service, TestRequest}, App,
    };
    use crate::database;
    use crate::database::models::Restaurant;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[actix_rt::test]
    async fn test_load_single_entry() {
        let db_pool = database::new_pool();
        println!("wow");

        let mut app = init_service(App::new()
        .data(db_pool.clone())
        .service(super::load_data))
        .await;
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

        let restaurant_vec = Restaurant::get_all_resturants(&db_pool.get().expect("Cant get database connection"));

        println!("wow");

        assert_eq!(restaurant_vec.len(), 1);
    }


    #[actix_rt::test]
    async fn test_remote_connection() {
        let db_pool = database::new_pool();

        let mut app = init_service(App::new()
        .data(db_pool.clone())
        .service(super::load_data))
        .await;

        //Send a request with a single restaurant in the body in json format
        TestRequest::get()
            .uri("/admin/insert")
            .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(54, 231, 65, 23)), 8080))
            .method(Method::POST)
            .set_payload("[    {        \"cvrnr\": \"27560946\",        \"pnr\": \"1012180825\",        \"region\": null,        \"industry_code\": \"472400\",        \"industry_text\": \"Detailhandel med brød, konditori- og sukkervarer\",        \"start_date\": \"2005-06-01T00:00:00Z\",        \"smiley_reports\": [            {                \"report_id\": \"Virk1873390\",                \"smiley\": 1,                \"date\": \"2021-03-22T00:00:00Z\"            },            {                \"report_id\": \"Virk1834536\",                \"smiley\": 1,                \"date\": \"2020-11-11T00:00:00Z\"            },            {                \"report_id\": \"Virk1789610\",                \"smiley\": 1,                \"date\": \"2020-08-04T00:00:00Z\"            },            {                \"report_id\": \"Virk1771244\",                \"smiley\": 2,                \"date\": \"2020-06-10T00:00:00Z\"            }        ],        \"city\": \"Charlottenlund\",        \"elite_smiley\": \"0\",        \"geo_lat\": 55.762464,        \"geo_lng\": 12.585801,        \"franchise_name\": null,        \"niche_industry\": \"Bagere og bagerafdelinger\",        \"url\": \"http://www.findsmiley.dk/da-DK/Searching/DetailsView.htm?virk=757164\",        \"address\": \"Ordrup Jagtvej 42B, st\",        \"name\": \"Patricks Bake Shop - Ordrup ApS\",        \"name_seq_nr\": \"757164\",        \"zip_code\": \"2920\",        \"ad_protection\": \"0\",        \"company_type\": \"Detail\"    }]".as_bytes())
            .send_request(&mut app)
            .await;

        let restaurant_vec = Restaurant::get_all_resturants(&db_pool.get().expect("Cant get database connection"));

        assert_eq!(restaurant_vec.len(), 0);
    }
}
