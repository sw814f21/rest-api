//cfg(test) tells compiler only to use this when running tests
#[cfg(test)]
mod tests {
    use crate::tests::response_parser;
    use crate::utils::data_inserter::*;
    use crate::utils::json_parser;
    use crate::{database::*, services, utils::data_loader::load_data_from_file};
    use actix_web::{test, web, App};
    use append_smiley::RestaurantWithSmileyReport;
    use diesel::prelude::*;
    use models::{Restaurant, SmileyReport};

    use crate::database::models::Version;
    use crate::database::new_pool;
    use diesel::{sqlite::SqliteConnection, QueryDsl, RunQueryDsl};

    fn load_test_data(conn: &SqliteConnection) {
        load_data_from_file(&String::from("test_sample_data.json"), conn);
    }

    #[actix_rt::test]
    async fn test_insert_restaurant() {
        let conn = new_pool().get().unwrap();

        let version = Version::get_from_token(&conn, "1");

        let testres = json_parser::JsonRestaurant {
            city: String::from("test"),
            cvr: String::from("15454331"),
            latitude: 32.0,
            longitude: 13.0,
            pnr: String::from("64848234"),
            address: String::from("someting vej 3"),
            zipcode: String::from("3145"),
            name: String::from("Fishing fish grill"),
            smiley_restaurant_id: String::from("42545"),
            smiley_reports: Vec::new(),
            region: Some(String::from("abba")),
            industry_code: String::from("abba"),
            industry_text: String::from("abba"),
            start_date: String::from("abba"),
            end_date: "".to_string(),
            elite_smiley: String::from("abba"),
            niche_industry: String::from("abba"),
            url: String::from("abba"),
            ad_protection: String::from("abba"),
            company_type: String::from("abba"),
            franchise_name: Some(String::from("abba")),
        };

        let testres2 = map_restaurant_json2insert(&testres, version.id);

        let inserts = vec![testres2];

        insert_restaurants(&conn, &inserts);
        match schema::restaurant::dsl::restaurant
            .filter(schema::restaurant::smiley_restaurant_id.eq("42545"))
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
            smiley_restaurant_id: String,
            cvr: String,
            pnr: String,
        }
        let expected = vec![
            Vals {
                smiley_restaurant_id: String::from("69908"),
                cvr: String::from("29367876"),
                pnr: String::from("1012127266"),
            },
            Vals {
                smiley_restaurant_id: String::from("710347"),
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
                expected.get(j).unwrap().smiley_restaurant_id
            );
            assert_eq!(i.cvr, expected.get(j).unwrap().cvr);
            assert_eq!(i.pnr, expected.get(j).unwrap().pnr);
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
        let resp: response_parser::Restaurantandsmiley = test::read_body_json(resp).await;
        assert_eq!(resp.id, 5);
        assert_eq!(resp.smiley_restaurant_id, "758030");
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
        let mut resp: Vec<response_parser::Restaurantandsmiley> = test::read_body_json(resp).await;
        resp.sort_by(|a, b| {
            a.smiley_restaurant_id
                .partial_cmp(&b.smiley_restaurant_id)
                .unwrap()
        });

        assert_eq!(resp.len(), 3);

        let mut first_found: bool = false;
        let mut second_found: bool = false;
        let mut third_found: bool = false;

        for res in resp {
            match res.smiley_restaurant_id.as_str() {
                "659918" => {
                    assert_eq!(res.cvr, "36545860");
                    assert_eq!(res.pnr, "1020169008");
                    first_found = true;
                }
                "69908" => {
                    assert_eq!(res.cvr, "29367876");
                    assert_eq!(res.pnr, "1012127266");
                    second_found = true;
                }
                "47738" => {
                    assert_eq!(res.cvr, "30138929");
                    assert_eq!(res.pnr, "1000765515");
                    third_found = true;
                }
                _ => {} //Ignore the rest
            }
        }
        assert!(first_found);
        assert!(second_found);
        assert!(third_found);
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
        assert_eq!(resp.smiley_restaurant_id, "717825");
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

    #[actix_rt::test]
    async fn test_unsubscribing() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(services::subscription::unsubscribe),
        )
        .await;

        let input = services::subscription::SubscriptionRequest {
            restaurant_id: 5,
            token: String::from("i like this"),
        };

        diesel::insert_into(schema::subscription::table)
            .values(&input)
            .execute(&pool.get().unwrap())
            .expect("error preparing data for unsubscribing");

        let req = test::TestRequest::delete()
            .uri("/unsubscribe")
            .set_json(&input)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let lookup = schema::subscription::dsl::subscription
            .filter(schema::subscription::dsl::restaurant_id.eq_all(input.restaurant_id))
            .filter(schema::subscription::dsl::token.eq_all(input.token))
            .get_results::<models::Subscription>(&pool.get().unwrap())
            .expect("error looking for test subscription");
        assert_eq!(lookup.iter().count(), 0);
    }

    #[actix_rt::test]
    async fn test_get_smiley_reports_by_restaurant_id() {
        let pool = new_pool().get().unwrap();
        load_test_data(&pool);
        let res = models::SmileyReport::get_smiley_reports_for_id(2, &pool);
        assert_eq!(res.iter().count(), 4);
        let mut expected: Vec<SmileyReport> = vec![
            SmileyReport {
                id: 8,
                res_id: 2,
                rating: 1,
                report_id: String::from("Virk1597065"),
                date: String::from("2019-03-27T00:00:00Z"),
            },
            SmileyReport {
                id: 7,
                res_id: 2,
                rating: 1,
                report_id: String::from("Virk1629508"),
                date: String::from("2019-06-27T00:00:00Z"),
            },
            SmileyReport {
                id: 6,
                res_id: 2,
                rating: 1,
                report_id: String::from("Virk1774060"),
                date: String::from("2020-06-17T00:00:00Z"),
            },
            SmileyReport {
                id: 5,
                res_id: 2,
                rating: 1,
                report_id: String::from("Virk1827526"),
                date: String::from("2020-10-28T00:00:00Z"),
            },
        ];
        for r in res {
            let vals = expected.pop().unwrap();
            assert_eq!(r.id, vals.id);
            assert_eq!(r.res_id, vals.res_id);
            assert_eq!(r.rating, vals.rating);
            assert_eq!(r.report_id, vals.report_id);
            assert_eq!(r.date, vals.date);
        }
    }

    #[actix_rt::test]
    async fn test_append_smiley() {
        let test_data: Vec<(Restaurant, SmileyReport)> = vec![
            (
                Restaurant {
                    id: 1,
                    smiley_restaurant_id: String::from("1"),
                    name: String::from(""),
                    address: String::from(""),
                    zipcode: String::from(""),
                    city: String::from(""),
                    cvr: String::from(""),
                    pnr: String::from(""),
                    latitude: 0.0,
                    longitude: 0.0,
                    version_number: 1,
                    region: Some(String::from("abba")),
                    industry_code: String::from("abba"),
                    industry_text: String::from("abba"),
                    start_date: String::from("abba"),
                    end_date: "".to_string(),
                    elite_smiley: String::from("abba"),
                    niche_industry: String::from("abba"),
                    url: String::from("abba"),
                    ad_protection: String::from("abba"),
                    company_type: String::from("abba"),
                    franchise_name: Some(String::from("abba")),
                },
                SmileyReport {
                    id: 1,
                    res_id: 1,
                    rating: 1,
                    report_id: String::from(""),
                    date: String::from("1"),
                },
            ),
            (
                Restaurant {
                    id: 1,
                    smiley_restaurant_id: String::from("1"),
                    name: String::from(""),
                    address: String::from(""),
                    zipcode: String::from(""),
                    city: String::from(""),
                    cvr: String::from(""),
                    pnr: String::from(""),
                    latitude: 0.0,
                    longitude: 0.0,
                    version_number: 1,
                    region: Some(String::from("abba")),
                    industry_code: String::from("abba"),
                    industry_text: String::from("abba"),
                    start_date: String::from("abba"),
                    end_date: "".to_string(),
                    elite_smiley: String::from("abba"),
                    niche_industry: String::from("abba"),
                    url: String::from("abba"),
                    ad_protection: String::from("abba"),
                    company_type: String::from("abba"),
                    franchise_name: Some(String::from("abba")),
                },
                SmileyReport {
                    id: 2,
                    res_id: 1,
                    rating: 1,
                    report_id: String::from(""),
                    date: String::from("2"),
                },
            ),
            (
                Restaurant {
                    id: 2,
                    smiley_restaurant_id: String::from("2"),
                    name: String::from(""),
                    address: String::from(""),
                    zipcode: String::from(""),
                    city: String::from(""),
                    cvr: String::from(""),
                    pnr: String::from(""),
                    latitude: 0.0,
                    longitude: 0.0,
                    version_number: 1,
                    region: Some(String::from("abba")),
                    industry_code: String::from("abba"),
                    industry_text: String::from("abba"),
                    start_date: String::from("abba"),
                    end_date: "".to_string(),
                    elite_smiley: String::from("abba"),
                    niche_industry: String::from("abba"),
                    url: String::from("abba"),
                    ad_protection: String::from("abba"),
                    company_type: String::from("abba"),
                    franchise_name: Some(String::from("abba")),
                },
                SmileyReport {
                    id: 3,
                    res_id: 2,
                    rating: 1,
                    report_id: String::from(""),
                    date: String::from("1"),
                },
            ),
        ];
        let expected: Vec<RestaurantWithSmileyReport> = vec![
            RestaurantWithSmileyReport {
                id: 1,
                smiley_restaurant_id: String::from("1"),
                name: String::from(""),
                address: String::from(""),
                zipcode: String::from(""),
                city: String::from(""),
                cvr: String::from(""),
                pnr: String::from(""),
                latitude: 0.0,
                longitude: 0.0,
                version_number: 1,
                smileyreports: vec![
                    SmileyReport {
                        id: 1,
                        res_id: 1,
                        rating: 1,
                        report_id: String::from(""),
                        date: String::from("1"),
                    },
                    SmileyReport {
                        id: 2,
                        res_id: 1,
                        rating: 1,
                        report_id: String::from(""),
                        date: String::from("2"),
                    },
                ],
            },
            RestaurantWithSmileyReport {
                id: 2,
                smiley_restaurant_id: String::from("2"),
                name: String::from(""),
                address: String::from(""),
                zipcode: String::from(""),
                city: String::from(""),
                cvr: String::from(""),
                pnr: String::from(""),
                latitude: 0.0,
                longitude: 0.0,
                version_number: 1,
                smileyreports: vec![SmileyReport {
                    id: 3,
                    res_id: 2,
                    rating: 1,
                    report_id: String::from(""),
                    date: String::from("1"),
                }],
            },
        ];
        let res = append_smiley::convert_res_smiley_pairs(test_data);
        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_append_smiley_empty() {
        let test_data: Vec<(Restaurant, SmileyReport)> = Vec::new();
        let res = append_smiley::convert_res_smiley_pairs(test_data);
        assert_eq!(res.len(), 0);
    }
}
