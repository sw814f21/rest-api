//cfg(test) tells compiler only to use this when running tests
#[cfg(test)]
mod tests {
    use crate::database::schema::*;
    use crate::utils::data_inserter::*;
    use crate::{database::models::*, utils::data_loader::load_data};
    use actix_web::http::Method;
    use actix_web::{
        test::{init_service, read_body, TestRequest},
        web, App,
    };
    use diesel::prelude::*;

    use crate::database::new_pool;
    use crate::diesel::connection::SimpleConnection;
    use diesel::r2d2::CustomizeConnection;
    use diesel::{
        dsl::{delete, exists, insert_into, select},
        r2d2::PooledConnection,
    };
    use diesel::{
        r2d2::{self, ConnectionManager, Pool},
        sqlite::SqliteConnection,
        QueryDsl, RunQueryDsl,
    };

    fn load_test_data(conn: &SqliteConnection) {
        load_data(&String::from("test_sample_data.json"), conn);
    }

    async fn clear_database(conn: &SqliteConnection) {
        let _ = diesel::delete(restaurant::table).execute(conn);
        let _ = diesel::delete(notification_history::table).execute(conn);
        let _ = diesel::delete(smiley_report::table).execute(conn);
        let _ = diesel::delete(subscription::table).execute(conn);
    }

    #[actix_rt::test]
    async fn test_insert_restaurant() {
        let pool = new_pool();
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
        let testid = insert_restaurant(&pool.get().unwrap(), &testres);
        match restaurant::dsl::restaurant
            .filter(restaurant::smiley_restaurant_id.eq(testres.smiley_restaurant_id))
            .filter(restaurant::name.eq_all(testres.name))
            .filter(restaurant::address.eq_all(testres.address))
            .filter(restaurant::zipcode.eq_all(testres.zipcode))
            .filter(restaurant::city.eq_all(testres.city))
            .filter(restaurant::cvr.eq_all(testres.cvr))
            .filter(restaurant::pnr.eq_all(testres.pnr))
            .filter(restaurant::latitude.eq_all(testres.latitude))
            .filter(restaurant::longitude.eq_all(testres.longitude))
            .first::<Restaurant>(&pool.get().unwrap())
        {
            Err(_) => panic!("Error in test for insert of test restaurant"),
            Ok(res) => assert_eq!(res.id, testid),
        }
    }

    #[actix_rt::test]
    async fn data_loading_test() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let addedres: Vec<Restaurant> = restaurant::dsl::restaurant
            .order_by(restaurant::dsl::id)
            .load(&pool.get().unwrap())
            .expect("error fetching testdata restaurants in test");
        let addedsmileyreports: Vec<SmileyReport> = smiley_report::dsl::smiley_report
            .order_by((smiley_report::dsl::restaurant_id, smiley_report::dsl::date))
            .load(&pool.get().unwrap())
            .expect("error fetching smiley reports in test");
        assert_eq!(addedres.iter().count(), 10);
        assert_eq!(addedsmileyreports.iter().count(), 40);
    }

    #[actix_rt::test]
    async fn lat_lng_test() {
        let pool = new_pool();
        load_test_data(&pool.get().unwrap());
        let mut res = Restaurant::search_by_lat_lng(55.9, 9.0, 55.2, 10.1, &pool.get().unwrap());
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
}
