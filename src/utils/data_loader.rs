use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

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
    let r: HashMap<String, RestaurantData> = serde_json::from_reader(reader).expect("Can't parse json");

    for (k,v) in r {
        match v.city {
            Some(p) => println!("{} {}", k, p),
            None => println!("City is null")
        }
        
    }
}

