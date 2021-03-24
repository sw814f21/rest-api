use diesel::prelude::*;

use super::models::NewRestaurant;
use super::schema::restaurants;

pub fn insert_restaurants(conn: &SqliteConnection, restaurants_data: &Vec<NewRestaurant>) -> usize {
    diesel::insert_into(restaurants::table)
        .values(restaurants_data)
        .execute(conn)
        .expect("Error saving new restaurant")
}
