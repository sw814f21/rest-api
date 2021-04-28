use super::json_parser::{DeleteData, JsonRestaurant, JsonSmileyReport};
use crate::utils::json_parser::RichData;
use crate::{database::models::Version, services::restaurant};
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
    println!("Importing new data...");
    let read_json: RichData = serde_json::from_str(json).expect("Unable to parse insert json");

    let ver = Version::get_from_token(connection, &read_json.token);

    for res in read_json.data {
        let resid = insert_restaurant(&connection, &res, ver.id);

        insert_smileys(&connection, &res.smiley_reports, resid);
    }
}

pub fn update_smiley_data(json: &String, connection: &SqliteConnection) {
    println!("Importing updated data...");
    let read_json: RichData = serde_json::from_str(json).expect("Unable to parse update json");

    let ver = Version::get_from_token(connection, &read_json.token);

    let restaurants = read_json.data;

    for res in restaurants {
        let resid: i32 = update_restaurant(&connection, &res, ver.id);
        for report in &res.smiley_reports {
            update_smileys(&connection, &report, resid);
        }
    }
}

pub fn get_data_from_database(conn: &SqliteConnection) -> Vec<JsonRestaurant> {
    use schema::*;
    println!("Dumping data from database..");

    // We join on the restaurant ID on the restaurant table and smiley_report table
    let joined_smiley_report_restaurnt = restaurant::table
        .inner_join(smiley_report::table.on(smiley_report::restaurant_id.eq(restaurant::id)))
        .load::<(Restaurant, SmileyReport)>(conn)
        .unwrap();

    conv_res_smiley_to_jsonres(joined_smiley_report_restaurnt)
}

pub fn conv_res_smiley_to_jsonres(data: Vec<(Restaurant, SmileyReport)>) -> Vec<JsonRestaurant> {
    // Create a variable
    let mut result = Vec::new();
    let mut smiley_id = 0;

    // if the joined smiley report tuple is not empty, we add the first restaurant
    if !data.is_empty() {
        // grab the restaurant part of the joined smiley report
        let joined_smiley_report_tuple = data.get(0).unwrap();

        // set smiley id
        smiley_id = joined_smiley_report_tuple.0.smiley_restaurant_id;

        // convert the first restaurant  to a JSON restaurant
        let mut first_json_restaurant = restaurant_to_jsonrestaurant(&joined_smiley_report_tuple.0);

        // push the smiley reports to the JSON restaurant
        first_json_restaurant
            .smiley_reports
            .push(smileyreport_to_jsonsmileyreport(
                &joined_smiley_report_tuple.1,
            ));

        // push the first restaurant
        result.push(first_json_restaurant);
    }

    // iterate over joined smiley reports
    for i in 1..data.len() {
        // grab a joined smiley report tuple
        let joined_smiley_report_restaurant = data.get(i).unwrap();

        // put the restaurant and smiley reports into variables
        let current_restaurant = &joined_smiley_report_restaurant.0;
        let current_smiley_report = &joined_smiley_report_restaurant.1;

        // if the smiley id does not match the current restaurant, we got a new restaurant
        if smiley_id != current_restaurant.smiley_restaurant_id {
            smiley_id = current_restaurant.smiley_restaurant_id;
            result.push(restaurant_to_jsonrestaurant(current_restaurant));
        }

        // push the new smiley report to the restaurant
        result
            .last_mut()
            .unwrap()
            .smiley_reports
            .push(smileyreport_to_jsonsmileyreport(current_smiley_report));
    }

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
        region: restaurant.region.clone(),
        industry_code: (*restaurant.industry_code).to_string(),
        industry_text: (*restaurant.industry_text).to_string(),
        start_date: (*restaurant.start_date).to_string(),
        elite_smiley: (*restaurant.elite_smiley).to_string(),
        niche_industry: (*restaurant.niche_industry).to_string(),
        url: (*restaurant.url).to_string(),
        ad_protection: (*restaurant.ad_protection).to_string(),
        company_type: (*restaurant.company_type).to_string(),
        franchise_name: restaurant.franchise_name.clone(),
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
