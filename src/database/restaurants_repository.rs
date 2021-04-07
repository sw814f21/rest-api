use diesel::prelude::*;

use super::schema::restaurants;
use super::schema::smileyreports;
use crate::utils::data_loader::NewRestaurant;
use crate::utils::data_loader::NewSmileyReport;

pub fn insert_restaurants(conn: &SqliteConnection, restaurants_data: &Vec<NewRestaurant>) -> usize {
    diesel::insert_into(restaurants::table)
        .values(restaurants_data)
        .execute(conn)
        .expect("Error saving new restaurant")
}

pub fn insert_smileys(conn: &SqliteConnection, smiley_data: &Vec<NewSmileyReport>) -> usize {
    diesel::insert_into(smileyreports::table)
        .values(smiley_data)
        .execute(conn)
        .expect("Error saving new smiley data")
}
