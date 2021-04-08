use std::borrow::Borrow;

use diesel::prelude::*;

use super::schema::restaurants;
use super::schema::restaurants::dsl::restaurants as r_dsl;
use super::schema::restaurants::id;
use super::schema::restaurants::latitude;
use super::schema::restaurants::longitude;
use super::schema::restaurants::url;
use super::schema::smileyreports;
use crate::utils::data_loader::NewRestaurant;
use crate::utils::data_loader::NewSmileyReport;

pub fn insert_restaurants(conn: &SqliteConnection, restaurants_data: NewRestaurant) -> i32 {
    diesel::insert_into(restaurants::table)
        .values(restaurants_data.borrow())
        .execute(conn)
        .expect("Error saving new restaurant");
    r_dsl
        .filter(latitude.eq_all(restaurants_data.latitude.borrow()))
        .filter(longitude.eq_all(restaurants_data.longitude.borrow()))
        .filter(url.like::<String>(restaurants_data.url.to_string()))
        .select(id)
        .first(conn)
        .expect("err")
}

pub fn insert_smileys(conn: &SqliteConnection, smiley_data: &Vec<NewSmileyReport>) -> usize {
    diesel::insert_into(smileyreports::table)
        .values(smiley_data)
        .execute(conn)
        .expect("Error saving new smiley data")
}
