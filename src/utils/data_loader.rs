use crate::database::new_pool;
use crate::utils::data_inserter::{
    insert_restaurant, insert_smileys, InsertRestaurant, InsertSmileyReport,
};
use crate::utils::json_parser::{JsonRestaurant, JsonSmileyReport};
use std::fs::File;
use std::io::BufReader;

pub fn load_data(path: &String) {
    let file = File::open(path).expect("Can't open file from path");
    let reader = BufReader::new(file);
    let read_json: Vec<JsonRestaurant> = serde_json::from_reader(reader).expect("Can't parse json");

    let connection_pool = new_pool();
    let connection = connection_pool.get().expect("Can't get connection");

    for res in read_json {
        let new_restaurant = map_restaurant_json2insert(&res);
        let resid = insert_restaurant(&connection, &new_restaurant);

        let mut newsmileyreports: Vec<InsertSmileyReport> = Vec::new();

        for report in &res.smiley_reports {
            newsmileyreports.push(map_smileyreport_json2insert(&report, resid));
        }

        insert_smileys(&connection, &newsmileyreports);
    }
    println!("Finished loading data into database")
}

fn map_restaurant_json2insert(input: &JsonRestaurant) -> InsertRestaurant {
    InsertRestaurant {
        smiley_restaurant_id: input.smiley_restaurant_id.parse::<i32>().unwrap(),
        name: (*input.name).to_string(),
        address: (*input.address).to_string(),
        zipcode: (*input.zipcode).to_string(),
        city: (*input.city).to_string(),
        cvr: (*input.cvr).to_string(),
        pnr: (*input.pnr).to_string(),
        latitude: input.latitude,
        longitude: input.longitude,
    }
}

fn map_smileyreport_json2insert(input: &JsonSmileyReport, res_id: i32) -> InsertSmileyReport {
    InsertSmileyReport {
        date: (*input.date).to_string(),
        smiley: input.smiley,
        restaurant_id: res_id,
        report_id: (*input.report_id).to_string(),
    }
}