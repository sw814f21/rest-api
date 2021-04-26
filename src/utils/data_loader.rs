use super::json_parser::{DeleteData, JsonRestaurant, JsonSmileyReport};
use crate::{database::models::Version, services::restaurant};
use crate::utils::json_parser::RichData;
use crate::{
    database::schema,
    utils::data_inserter::{
        insert_restaurant, insert_smileys, remove_restaurant, update_restaurant, update_smileys,
    },
};
use diesel::{JoinOnDsl, QueryDsl, SqliteConnection};

use diesel::prelude::*;

use crate::database::models::*;

pub fn load_data_from_file(path: &String, conn: &SqliteConnection) {
    let json = std::fs::read_to_string(path).expect("Failed to read file");

    insert_smiley_data(&json, conn);

    println!("Finished loading data into database");
}

pub fn insert_smiley_data(json: &String, connection: &SqliteConnection) {
    let read_json: RichData = serde_json::from_str(json).expect("wut");

    let ver = Version::get_from_token(connection, &read_json.token);

    for res in read_json.data {
        let resid = insert_restaurant(&connection, &res, ver.id);

        insert_smileys(&connection, &res.smiley_reports, resid);
    }
}

pub fn update_smiley_data(json: &String, connection: &SqliteConnection) {
    let read_json: RichData = serde_json::from_str(json).expect("Can't parse json");

    let ver = Version::get_from_token(connection, &read_json.token);

    let restaurants = read_json.data;

    for res in restaurants {
        let resid: i32 = update_restaurant(&connection, &res, ver.id);
        for report in &res.smiley_reports {
            update_smileys(&connection, &report, resid);
        }
    }
}

pub fn get_data(conn: &SqliteConnection) -> Vec<&mut JsonRestaurant> {
    use schema::*;

    // We join on the restaurant ID on the restaurant table and smiley_report table
    let joined_smiley_report_restaurnt = restaurant::table
        .inner_join(smiley_report::table.on(smiley_report::restaurant_id.eq(restaurant::id)))
        .load::<(Restaurant, SmileyReport)>(conn)
        .unwrap();

    // (LONG COMMENTS, TO BE DELETED) We iterate for the sake of the example.
    // We can already at this point simply return the vector from the function

    // Iterate over the vector. Note that we have two objects, the smiley report and restaurants.
    // The reason behind the two objects is that the two tables are joined, hence
    // we can access both values.
    let mut result: Vec<&mut JsonRestaurant> = Vec::new();
    
    if !joined_smiley_report_restaurnt.is_empty() {
        let first_ele = &joined_smiley_report_restaurnt[0].0;

        let mut current_json_res: &mut JsonRestaurant;
        let mut first = restaurant_to_jsonrestaurant(first_ele);
        result.push(&mut first);
        current_json_res = result[0];
        let mut old_restaurant: &Restaurant = first_ele;
    

        for row in &joined_smiley_report_restaurnt {
            let current_restaurant = &row.0;
            let current_smiley_report = &row.1;

            if current_restaurant.smiley_restaurant_id != old_restaurant.smiley_restaurant_id {
                result.push(&mut restaurant_to_jsonrestaurant(current_restaurant));
                let abba = result.len();
                current_json_res = result[abba];
                //Create new restaurant & add to result
                old_restaurant = &current_restaurant;
            }
            (*current_json_res).smiley_reports.push(smileyreport_to_jsonsmileyreport(current_smiley_report));
            //append smiley to current restaurant.    

        }
    }

    // Return the joined result
    result
}

fn restaurant_to_jsonrestaurant(restaurant: &Restaurant) -> JsonRestaurant {
    JsonRestaurant {
        address: (*restaurant.address).to_string(),
        city: (*restaurant.city).to_string(),
        cvr: (*restaurant.cvr).to_string(),
        latitude: restaurant.latitude,
        longitude: restaurant.longitude,
        name: (*restaurant.name).to_string(),
        pnr: (*restaurant.pnr).to_string(),
        smiley_reports: Vec::new(),
        smiley_restaurant_id: restaurant.smiley_restaurant_id.to_string(),
        zipcode: (*restaurant.zipcode).to_string(),
    }
}

fn smileyreport_to_jsonsmileyreport(report: &SmileyReport) -> JsonSmileyReport {
    JsonSmileyReport {
        date: (*report.date).to_string(),
        report_id: (*report.report_id).to_string(),
        smiley: report.rating,
    }
}

pub fn delete_smiley_records(json: &String, connection: &SqliteConnection) {
    let data_to_delete: DeleteData = serde_json::from_str(json).expect("Can't parse json");
    let ver = Version::get_from_token(connection, &data_to_delete.token);
    remove_restaurant(&connection, data_to_delete.data, &ver);
}
