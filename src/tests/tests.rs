//cfg(test) tells compiler only to use this when running tests
#[cfg(test)]
mod tests {
    use crate::tests::response_parser;
    use crate::utils::data_inserter::*;
    use crate::{database::*, services, utils::data_loader::load_data};
    use actix_web::{test, web, App};
    use diesel::prelude::*;

    use crate::database::new_pool;
    use diesel::{sqlite::SqliteConnection, QueryDsl, RunQueryDsl};

    fn load_test_data(conn: &SqliteConnection) {
        load_data(&String::from("test_sample_data.json"), conn);
    }

    #[actix_rt::test]
    async fn test_insert_restaurant() {
        let conn = new_pool().get().unwrap();
        let testres: InsertRestaurant = InsertRestaurant {
            smiley_restaurant_id: 1,
            name: String::from("Somewhere"),
            address: String::from("Over"),
            zipcode: String::from("The"),
            city: String::from("Rainbow"),
            cvr: String::from("Is"),
            pnr: String::from("Happiness"),
            latitude: 1.5,
            longitude: 55.2,
        };
        let testid = insert_restaurant(&conn, &testres);
        match schema::restaurant::dsl::restaurant
            .filter(schema::restaurant::smiley_restaurant_id.eq(testres.smiley_restaurant_id))
            .filter(schema::restaurant::name.eq_all(testres.name))
            .filter(schema::restaurant::address.eq_all(testres.address))
            .filter(schema::restaurant::zipcode.eq_all(testres.zipcode))
            .filter(schema::restaurant::city.eq_all(testres.city))
            .filter(schema::restaurant::cvr.eq_all(testres.cvr))
            .filter(schema::restaurant::pnr.eq_all(testres.pnr))
            .filter(schema::restaurant::latitude.eq_all(testres.latitude))
            .filter(schema::restaurant::longitude.eq_all(testres.longitude))
            .get_results::<models::Restaurant>(&conn)
        {
            Err(_) => panic!("Error in test for insert of test restaurant"),
            Ok(res) => {
                assert_eq!(res.iter().count(), 1);
                assert_eq!(res.get(0).unwrap().id, testid)
            }
        }
    }

    #[actix_rt::test]
    async fn data_loading_test() {
        let conn = new_pool().get().unwrap();
        load_test_data(&conn);
        let addedres: Vec<models::Restaurant> = schema::restaurant::dsl::restaurant
            .order_by(schema::restaurant::dsl::id)
            .load(&conn)
            .expect("error fetching testdata restaurants in test");
        let addedsmileyreports: Vec<models::SmileyReport> =
            schema::smiley_report::dsl::smiley_report
                .order_by((
                    schema::smiley_report::dsl::restaurant_id,
                    schema::smiley_report::dsl::date,
                ))
                .load(&conn)
                .expect("error fetching smiley reports in test");
        assert_eq!(addedres.iter().count(), 10);
        assert_eq!(addedsmileyreports.iter().count(), 40);
    }

    #[actix_rt::test]
    async fn test_lat_lng() {
        let conn = new_pool().get().unwrap();
        load_test_data(&conn);
        let mut res = models::Restaurant::search_by_lat_lng(55.9, 9.0, 55.2, 10.1, &conn);
        struct Vals {
            smiley_restaurant_id: i32,
            cvr: String,
            pnr: String,
        }
        let excepted = vec![
            Vals {
                smiley_restaurant_id: 69908,
                cvr: String::from("29367876"),
                pnr: String::from("1012127266"),
            },
            Vals {
                smiley_restaurant_id: 710347,
                cvr: String::from("38290789"),
                pnr: String::from("1022046981"),
            },
        ];
        res.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        assert_eq!(res.iter().count(), 2);
        let mut j = 0;

        for i in res {
            assert_eq!(
                i.smiley_restaurant_id,
                excepted.get(j).unwrap().smiley_restaurant_id
            );
            assert_eq!(i.cvr, excepted.get(j).unwrap().cvr);
            assert_eq!(i.pnr, excepted.get(j).unwrap().pnr);
            j = j + 1;
        }
    }

    #[actix_rt::test]
    async fn test_get_restaurants() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(services::restaurant::restaurant),
        )
        .await;
        let req = test::TestRequest::get().uri("/restaurant").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let resp: Vec<response_parser::Simplerestaurant> = test::read_body_json(resp).await;
        assert_eq!(resp.iter().count(), 10);
    }

    #[actix_rt::test]
    async fn test_get_restaurants_id() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(services::restaurant::restaurant_by_id),
        )
        .await;
        let req = test::TestRequest::get().uri("/restaurant/5").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let resp: response_parser::Restaurant = test::read_body_json(resp).await;
        assert_eq!(resp.id, 5);
        assert_eq!(resp.smiley_restaurant_id, 758030);
        assert_eq!(resp.cvr, "25431944");
        assert_eq!(resp.pnr, "1008217579");
    }

    #[actix_rt::test]
    async fn test_restaurant_search_name() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(services::restaurant::search_restaurants),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/restaurant/search?name=bager")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let mut resp: Vec<response_parser::Restaurant> = test::read_body_json(resp).await;
        resp.sort_by(|a, b| {
            a.smiley_restaurant_id
                .partial_cmp(&b.smiley_restaurant_id)
                .unwrap()
        });
        assert_eq!(resp.iter().count(), 3);
        let first = resp.pop().unwrap();
        let second = resp.pop().unwrap();
        let third = resp.pop().unwrap();

        assert_eq!(first.smiley_restaurant_id, 659918);
        assert_eq!(first.cvr, "36545860");
        assert_eq!(first.pnr, "1020169008");

        assert_eq!(second.smiley_restaurant_id, 69908);
        assert_eq!(second.cvr, "29367876");
        assert_eq!(second.pnr, "1012127266");

        assert_eq!(third.smiley_restaurant_id, 47738);
        assert_eq!(third.cvr, "30138929");
        assert_eq!(third.pnr, "1000765515");
    }
    #[actix_rt::test]
    async fn test_restaurant_search_multiple_params() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(services::restaurant::search_restaurants),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/restaurant/search?name=n&city=d&location=55.7,12.3,55.5,12.7")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let resp: Vec<response_parser::Restaurant> = test::read_body_json(resp).await;
        assert_eq!(resp.iter().count(), 1);
        let resp = resp.get(0).unwrap();
        assert_eq!(resp.id, 4);
        assert_eq!(resp.smiley_restaurant_id, 717825);
        assert_eq!(resp.cvr, "31262208");
        assert_eq!(resp.pnr, "1022913332");
    }

    #[actix_rt::test]
    async fn test_subscribing() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(services::subscription::subscribe),
        )
        .await;

        let input = services::subscription::SubscriptionRequest {
            restaurant_id: 5,
            token: String::from("i like this"),
        };
        let req = test::TestRequest::post()
            .uri("/subscribe")
            .set_json(&input)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let lookup = schema::subscription::dsl::subscription
            .filter(schema::subscription::dsl::restaurant_id.eq_all(input.restaurant_id))
            .filter(schema::subscription::dsl::token.eq_all(input.token))
            .get_result::<models::Subscription>(&pool.get().unwrap())
            .expect("error looking for test subscription");
        assert_eq!(lookup.restaurant_id, 5);
        assert_eq!(lookup.token, String::from("i like this"));
    }
}
