use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use crate::database::establish_connection;
use crate::database::models;


#[derive(Serialize, Deserialize)]
struct RestaurantData {
    #[serde(alias = "By")]
    #[serde(skip_serializing_if = "Option::is_none")] 
    city: Option<String>,

    #[serde(alias = "cvrnr")]
    #[serde(skip_serializing_if = "Option::is_none")] 
    cvr: Option<String>,
}

pub fn load_data(path: &String){
    let file = File::open(path).expect("Can't open file from path");
    let reader = BufReader::new(file);
    let r: HashMap<String, models::NewRestaurant> = serde_json::from_reader(reader).expect("Can't parse json");

    let connection_pool = establish_connection();
    let connection = connection_pool.get().expect("Can't get connection");

    for (_,restaurant) in r {
        models::create_restaurant(&connection, restaurant);
    }
}

