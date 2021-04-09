use crate::database::new_pool;
use crate::database::schema::restaurant;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};


#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "restaurant"]
pub struct NewRestaurant {
    #[serde(alias = "By")]
    pub city: String,

    #[serde(alias = "cvrnr")]
    pub cvr: String,

    #[serde(alias = "Geo_Lat")]
    pub latitude: f32,

    #[serde(alias = "Geo_Lng")]
    pub longitude: f32,

    #[serde(alias = "pnr")]
    pub pnr: String,

    #[serde(alias = "adresse1")]
    pub address: String,

    #[serde(alias = "postnr")]
    pub zipcode: String,

    #[serde(alias = "navn1")]
    pub name: String,

    #[serde(alias = "navnelbnr")]
    pub smiley_restaurant_id: i32,
}

pub fn load_data(path: &String) {
    let file = File::open(path).expect("Can't open file from path");
    let reader = BufReader::new(file);
    let restaurants_vector: Vec<NewRestaurant> =
        serde_json::from_reader(reader).expect("Can't parse json");

    let connection_pool = new_pool();
    let connection = connection_pool.get().expect("Can't get connection");

    insert_restaurants(&connection, &restaurants_vector);

    println!("Finished loading data into database")
}


use diesel::prelude::*;


fn insert_restaurants(conn: &SqliteConnection, restaurants_data: &Vec<NewRestaurant>) -> usize {
    diesel::insert_into(restaurant::table)
        .values(restaurants_data)
        .execute(conn)
        .expect("Error saving new restaurant")
}
