use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use diesel::dsl::{delete, select, insert_into, exists};

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
        res_dsl.select(smiley_restaurant_id)
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
            .filter(name.like(query + "%"))
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
            .filter(city.like(query + "%"))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error searching for restaurants with city")
    }
}

use super::schema::subscription;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "subscription"]
pub struct Subscription {
    pub id: i32,
    pub restaurant_id: i32,
    pub token: String,
}
impl Subscription {
    
    pub fn subscribe(res_id: i32, token_id: &String, conn: &SqliteConnection) {
        use crate::database::schema::subscription::dsl::*;
        let exists = select(exists(subscription.filter(
            (restaurant_id.eq(res_id)).and(token.eq(token_id))
        )
        )).first::<bool>(conn);
        match exists {
            Ok(_) => {},
            Err(_) => {
                let new_sub = (
                    restaurant_id.eq(res_id),
                    token.eq(token_id),
                );
                insert_into(subscription).values(&new_sub).execute(conn).expect("Couldn't create subscription");
            }
        }
    }

    pub fn unsubscribe(res_id: i32, token_id: &String, conn: &SqliteConnection) {
        use crate::database::schema::subscription::dsl::*;
        delete(subscription.filter(
            (restaurant_id.eq(res_id)).and(token.eq(token_id))
        )).execute(conn).expect("Couldn't delete subscription");
    }
}