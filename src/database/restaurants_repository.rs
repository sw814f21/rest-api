use diesel::prelude::*;

use super::schema::restaurants;
use super::models::NewRestaurant;

pub fn insert_restaurants(conn: &SqliteConnection, restaurants_data: &Vec<NewRestaurant>) -> usize {
    diesel::insert_into(restaurants::table)
        .values(restaurants_data)
        .execute(conn)
        .expect("Error saving new restaurant")
}