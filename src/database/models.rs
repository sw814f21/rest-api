use super::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use diesel::dsl::{delete, exists, insert_into, select};

#[derive(Clone, PartialEq, Queryable, Serialize)]
pub struct Restaurant {
    pub id: i32,
    pub smiley_restaurant_id: i32,
    pub name: String,
    pub address: String,
    pub zipcode: String,
    pub city: String,
    pub cvr: String,
    pub pnr: String,
    pub latitude: f32,
    pub longitude: f32,
    pub version_number: i32,
}

#[derive(Clone, PartialEq, Queryable, Serialize)]
pub struct SmileyReport {
    pub id: i32,
    pub res_id: i32,
    pub rating: i32,
    pub date: String,
    pub report_id: String,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Simplerestaurant {
    id: i32,
    lat: f32,
    lng: f32,
}

use super::schema::restaurant::dsl::restaurant as res_dsl;
impl Restaurant {
    pub fn get_restaurant_references(conn: &SqliteConnection) -> Vec<i32> {
        use super::schema::restaurant::dsl::smiley_restaurant_id;
        res_dsl
            .select(smiley_restaurant_id)
            .load::<i32>(conn)
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
    pub fn get_restaurant_by_id(res_id: i32, conn: &SqliteConnection) -> Restaurant {
        res_dsl
            .find(res_id)
            .get_result::<Restaurant>(conn)
            .ok()
            .expect("Error fetching restaurant ID")
    }

    pub fn get_restaurant_by_smiley_id(
        smiley_restaurant_id: i32,
        conn: &SqliteConnection,
    ) -> Restaurant {
        restaurant::table
            .filter(restaurant::smiley_restaurant_id.eq(smiley_restaurant_id))
            .load(conn)
            .expect("Failed to get restaurant")
            .remove(0)
    }

    pub fn search_by_lat_lng(
        nwlat: f32,
        nwlng: f32,
        selat: f32,
        selng: f32,
        conn: &SqliteConnection,
    ) -> Vec<Restaurant> {
        use super::schema::restaurant::dsl::latitude;
        use super::schema::restaurant::dsl::longitude;
        res_dsl
            .filter(latitude.lt(nwlat))
            .filter(latitude.gt(selat))
            .filter(longitude.gt(nwlng))
            .filter(longitude.lt(selng))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error fetching with Latitude/Longitude")
    }

    pub fn search_by_name(query: String, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::restaurant::dsl::name;
        res_dsl
            .filter(name.like("%".to_owned() + query.as_str() + "%"))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error searching with restaurant name")
    }

    pub fn search_by_zip(query: String, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::restaurant::dsl::zipcode;
        res_dsl
            .filter(zipcode.eq(query))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error searching for restaurants with zipcode")
    }

    pub fn search_by_city(query: String, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::restaurant::dsl::city;
        res_dsl
            .filter(city.like("%".to_owned() + query.as_str() + "%"))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error searching for restaurants with city")
    }

    pub fn get_since_version(conn: &SqliteConnection, version: i32) -> Vec<Restaurant> {
        restaurant::table
            .filter(restaurant::version_number.gt(version))
            .load::<Restaurant>(conn)
            .expect("Failed to get restaurants based on version")
    }
}
use crate::services::subscription::SubscriptionRequest;
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
}

impl Version {
    pub fn create_new_version(conn: &SqliteConnection) -> Version {
        insert_into(version_history::table)
            .default_values()
            .execute(conn)
            .expect("New version insertaton failed");

        version_history::table
            .order(version_history::id.desc())
            .first::<Version>(conn)
            .expect("Failed to get new version")
    }

    pub fn current_version(conn: &SqliteConnection) -> Version {
        version_history::table
            .order(version_history::id.desc())
            .first::<Version>(conn)
            .expect("Failed to get latest version")
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
