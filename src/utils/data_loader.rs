use crate::database::establish_connection;
use crate::database::models;
use crate::database::restaurants_repository;
use std::fs::File;
use std::io::BufReader;

pub fn load_data(path: &String) {
    let file = File::open(path).expect("Can't open file from path");
    let reader = BufReader::new(file);
    let restaurants_vector: Vec<models::NewRestaurant> =
        serde_json::from_reader(reader).expect("Can't parse json");

    let connection_pool = establish_connection();
    let connection = connection_pool.get().expect("Can't get connection");

    restaurants_repository::insert_restaurants(&connection, &restaurants_vector);

    println!("Finished loading data into database")
}
