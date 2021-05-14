use super::append_smiley::{convert_res_smiley_pairs, RestaurantWithSmileyReport};
use super::schema;
use super::schema::*;
use crate::services::subscription::SubscriptionRequest;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use diesel::dsl::{delete, exists, insert_into, select};

#[derive(Debug, Clone, PartialEq, Queryable, Serialize)]
pub struct Restaurant {
    pub id: i32,
    pub smiley_restaurant_id: String,
    pub name: String,
    pub address: String,
    pub zipcode: String,
    pub city: String,
    pub cvr: String,
    pub pnr: String,
    pub latitude: f64,
    pub longitude: f64,
    pub version_number: i32,
    pub region: Option<String>,
    pub industry_code: String,
    pub industry_text: String,
    pub start_date: String,
    pub end_date: String,
    pub elite_smiley: String,
    pub niche_industry: String,
    pub url: String,
    pub ad_protection: String,
    pub company_type: String,
    pub franchise_name: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Simplerestaurant {
    id: i32,
    lat: f64,
    lng: f64,
}

use super::schema::restaurant::dsl::restaurant as res_dsl;
impl Restaurant {
    pub fn get_restaurant_references(conn: &SqliteConnection) -> Vec<String> {
        use super::schema::restaurant::dsl::smiley_restaurant_id;
        res_dsl
            .select(smiley_restaurant_id)
            .load::<String>(conn)
            .expect("Error fetching ids from database")
    }

    pub fn get_all_resturants(conn: &SqliteConnection) -> Vec<Simplerestaurant> {
        use super::schema::restaurant::dsl::id;
        use super::schema::restaurant::dsl::latitude;
        use super::schema::restaurant::dsl::longitude;
        res_dsl
            .select((id, latitude, longitude))
            .load::<Simplerestaurant>(conn)
            .expect("Error fetching restaurant data")
    }
    pub fn get_restaurant_by_id(
        res_id: i32,
        conn: &SqliteConnection,
    ) -> RestaurantWithSmileyReport {
        let query = res_dsl
            .inner_join(schema::smiley_report::table)
            .filter(schema::restaurant::dsl::id.eq_all(res_id))
            .order(schema::smiley_report::date.asc())
            .get_results::<(Restaurant, SmileyReport)>(conn)
            .ok()
            .expect("Error fetching restaurant ID");
        convert_res_smiley_pairs(query).get(0).unwrap().to_owned()
    }

    pub fn get_restaurant_by_smiley_id(smiley_restaurant_id: &str, conn: &SqliteConnection) -> i32 {
        restaurant::table
            .filter(restaurant::smiley_restaurant_id.eq(smiley_restaurant_id))
            .select(restaurant::id)
            .first::<i32>(conn)
            .expect(
                format!(
                    "Failed to get restaurant with smiley id = {0}",
                    smiley_restaurant_id
                )
                .as_str(),
            )
    }

    pub fn search_by_lat_lng(
        nwlat: f64,
        nwlng: f64,
        selat: f64,
        selng: f64,
        conn: &SqliteConnection,
    ) -> Vec<RestaurantWithSmileyReport> {
        use super::schema::restaurant::dsl::latitude;
        use super::schema::restaurant::dsl::longitude;
        let query = res_dsl
            .inner_join(schema::smiley_report::table)
            .filter(latitude.lt(nwlat))
            .filter(latitude.gt(selat))
            .filter(longitude.gt(nwlng))
            .filter(longitude.lt(selng))
            .order((
                schema::restaurant::id.asc(),
                schema::smiley_report::date.asc(),
            ))
            .get_results::<(Restaurant, SmileyReport)>(conn)
            .ok()
            .expect("Error fetching with Latitude/Longitude");
        convert_res_smiley_pairs(query)
    }

    pub fn search_by_name(
        query: String,
        conn: &SqliteConnection,
    ) -> Vec<RestaurantWithSmileyReport> {
        use super::schema::restaurant::dsl::*;
        let query = res_dsl
            .inner_join(schema::smiley_report::table)
            .filter(name.like("%".to_owned() + query.as_str() + "%"))
            .order_by((id.asc(), schema::smiley_report::date.asc()))
            .get_results::<(Restaurant, SmileyReport)>(conn)
            .ok()
            .expect("Error searching with restaurant name");
        convert_res_smiley_pairs(query)
    }

    pub fn search_by_zip(
        query: String,
        conn: &SqliteConnection,
    ) -> Vec<RestaurantWithSmileyReport> {
        use super::schema::restaurant::dsl::zipcode;
        let query = res_dsl
            .inner_join(schema::smiley_report::table)
            .filter(zipcode.eq(query))
            .order((
                schema::restaurant::id.asc(),
                schema::smiley_report::date.asc(),
            ))
            .get_results::<(Restaurant, SmileyReport)>(conn)
            .ok()
            .expect("Error searching for restaurants with zipcode");
        convert_res_smiley_pairs(query)
    }

    pub fn search_by_city(
        query: String,
        conn: &SqliteConnection,
    ) -> Vec<RestaurantWithSmileyReport> {
        use super::schema::restaurant::dsl::city;
        let query = res_dsl
            .inner_join(schema::smiley_report::table)
            .filter(city.like("%".to_owned() + query.as_str() + "%"))
            .order((
                schema::restaurant::id.asc(),
                schema::smiley_report::date.asc(),
            ))
            .get_results::<(Restaurant, SmileyReport)>(conn)
            .ok()
            .expect("Error searching for restaurants with city");
        convert_res_smiley_pairs(query)
    }

    pub fn get_since_version(conn: &SqliteConnection, version: i32) -> Vec<Restaurant> {
        restaurant::table
            .filter(restaurant::version_number.gt(version))
            .load::<Restaurant>(conn)
            .expect("Failed to get restaurants based on version")
    }
}

#[derive(Debug, Clone, PartialEq, Queryable, Serialize)]
pub struct SmileyReport {
    pub id: i32,
    pub res_id: i32,
    pub rating: i32,
    pub report_id: String,
    pub date: String,
}

impl SmileyReport {
    pub fn get_smiley_reports_for_id(res_id: i32, conn: &SqliteConnection) -> Vec<SmileyReport> {
        use crate::database::schema::smiley_report::dsl::*;

        let mut query: Vec<SmileyReport> = smiley_report
            .filter(restaurant_id.eq_all(res_id))
            .get_results::<SmileyReport>(conn)
            .expect("Error fetching reports for restaurant id");

        query.sort_by(|a, b| b.date.partial_cmp(&a.date).unwrap());
        query
    }

    pub fn smiley_report_exists(res_id: i32, ireport_id: &str, conn: &SqliteConnection) -> bool {
        use crate::database::schema::smiley_report::dsl::*;

        let result = select(exists(
            smiley_report
                .filter(restaurant_id.eq(res_id))
                .filter(report_id.eq(ireport_id)),
        ))
        .get_result::<bool>(conn);

        match result {
            Ok(true) => true,
            Ok(false) => false,
            _ => panic!("Error testing if smiley report exists"),
        }
    }
}

#[derive(Clone, PartialEq, Queryable, Serialize)]
pub struct Subscription {
    pub id: i32,
    pub restaurant_id: i32,
    pub token: String,
}
impl Subscription {
    pub fn subscribe(request: SubscriptionRequest, conn: &SqliteConnection) {
        use crate::database::schema::subscription::dsl::*;
        let exists = select(exists(subscription.filter(
            (restaurant_id.eq(&request.restaurant_id)).and(token.eq(&request.token)),
        )))
        .get_result::<bool>(conn);
        match exists {
            Ok(x) => {
                if !x {
                    insert_into(subscription)
                        .values(&request)
                        .execute(conn)
                        .expect("Couldn't create subscription");
                }
            }
            Err(_) => {
                panic!("Error when looking for existing restaurant subscription");
            }
        }
    }

    pub fn unsubscribe(request: SubscriptionRequest, conn: &SqliteConnection) {
        use crate::database::schema::subscription::dsl::*;
        delete(
            subscription
                .filter((restaurant_id.eq(request.restaurant_id)).and(token.eq(request.token))),
        )
        .execute(conn)
        .expect("Couldn't delete subscription");
    }
}

#[table_name = "version_history"]
#[derive(Clone, PartialEq, Serialize, Queryable, QueryableByName)]
pub struct Version {
    pub id: i32,
    pub timestamp: String,
    pub token: String,
}

impl Version {
    pub fn current_version(conn: &SqliteConnection) -> Version {
        version_history::table
            .order(version_history::id.desc())
            .first::<Version>(conn)
            .expect("Failed to get latest version")
    }

    pub fn get_from_token(conn: &SqliteConnection, token_val: &str) -> Version {
        let exists = select(exists(
            version_history::table.filter(version_history::token.eq(token_val)),
        ))
        .get_result::<bool>(conn);
        match exists {
            Ok(true) => {
                let latest_token = version_history::table
                    .order(version_history::id.desc())
                    .select(version_history::token)
                    .first::<String>(conn)
                    .expect("Error fething the latest token version from the database");
                if latest_token != token_val {
                    panic!("Reuse of old version token!!");
                }
            }
            Ok(false) => {
                insert_into(version_history::table)
                    .values(version_history::token.eq(token_val))
                    .execute(conn)
                    .expect("Error inserting a token into the database");
            }
            Err(_) => {}
        }
        version_history::table
            .order(version_history::id.desc())
            .filter(version_history::token.eq(token_val))
            .first::<Version>(conn)
            .expect("Failed to fetch version from token")
    }
}

#[table_name = "removed_restaurant"]
#[derive(Clone, PartialEq, Serialize, Queryable, QueryableByName)]
pub struct RemovedRestaurant {
    pub restaurant_id: i32,
    pub version_number: i32,
}

impl RemovedRestaurant {
    pub fn get_removals_since(conn: &SqliteConnection, version: i32) -> Vec<RemovedRestaurant> {
        removed_restaurant::table
            .filter(removed_restaurant::version_number.gt(version))
            .load::<RemovedRestaurant>(conn)
            .expect("Failed to get removed restaurants")
    }
}
