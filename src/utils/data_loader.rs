use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use crate::database::establish_connection;
use crate::database::models;
use indicatif::ProgressBar;

pub fn load_data(path: &String){
    let file = File::open(path).expect("Can't open file from path");
    let reader = BufReader::new(file);
    let r: HashMap<String, models::NewRestaurant> = serde_json::from_reader(reader).expect("Can't parse json");

    let connection_pool = establish_connection();
    let connection = connection_pool.get().expect("Can't get connection");

    let pb = ProgressBar::new(r.len() as u64);
    for (_,restaurant) in r {
        models::create_restaurant(&connection, restaurant);
        pb.inc(1);
    }

    println!("Finished loading data into database")
}

