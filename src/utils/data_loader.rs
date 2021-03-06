use super::{
    data_inserter::{insert_smileys, map_smileyreport_json2insert, InsertSmileyReport},
    json_parser::{DeleteData, JsonRestaurant, JsonSmileyReport},
};
use crate::database::models::Version;
use crate::utils::json_parser::RichData;
use crate::{
    database::schema,
    utils::data_inserter::{
        insert_restaurants, map_restaurant_json2insert, remove_restaurant, update_restaurant,
        update_smileys, InsertRestaurant,
    },
};
use diesel::{JoinOnDsl, QueryDsl, SqliteConnection};

use diesel::prelude::*;

use crate::database::models::*;
use std::collections::HashMap;

pub fn load_data_from_file(path: &String, conn: &SqliteConnection) {
    let json = std::fs::read_to_string(path).expect("Failed to read file");

    insert_smiley_data(&json, conn);

    println!("Finished loading data into database");
}

pub fn insert_smiley_data(json: &String, connection: &SqliteConnection) {
    println!("Importing new data...");
    let read_json: RichData = serde_json::from_str(json).expect("Unable to parse insert json");

    let ver = Version::get_from_token(connection, &read_json.token);

    let restaurants: Vec<InsertRestaurant> = read_json
        .data
        .iter()
        .map(|r| map_restaurant_json2insert(r, ver.id))
        .collect();

    let id_map: HashMap<_, _> = insert_restaurants(&connection, &restaurants)
        .into_iter()
        .collect();

    let mut smiley_reports: Vec<InsertSmileyReport> = Vec::new();

    for res in read_json.data {
        match id_map.get(&res.smiley_restaurant_id) {
            Some(value) => {
                for rep in res.smiley_reports {
                    smiley_reports.push(map_smileyreport_json2insert(&rep, *value));
                }
            }
            None => {
                panic!("No match on the data which was just inserted!");
            }
        }
    }
    insert_smileys(&connection, &smiley_reports);
}

pub fn update_smiley_data(json: &String, connection: &SqliteConnection) {
    println!("Importing updated data...");
    let read_json: RichData = serde_json::from_str(json).expect("Unable to parse update json");

    let ver = Version::get_from_token(connection, &read_json.token);

    let restaurants = read_json.data;

    for res in restaurants {
        let resid = update_restaurant(&connection, &res, ver.id);
        for report in &res.smiley_reports {
            update_smileys(&connection, &report, resid);
        }
    }
}

pub fn get_smiley_data(conn: &SqliteConnection) -> Vec<JsonRestaurant> {
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
    let mut smiley_id = "".to_owned();

    // if the joined smiley report tuple is not empty, we add the first restaurant
    if !data.is_empty() {
        // grab the restaurant part of the joined smiley report
        let joined_smiley_report_tuple = data.get(0).unwrap();

        // set smiley id
        smiley_id = joined_smiley_report_tuple.0.smiley_restaurant_id.clone();

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
            smiley_id = current_restaurant.smiley_restaurant_id.clone();
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

#[actix_rt::test]
async fn res_to_jsonres_test() {
    let mut vec: Vec<(Restaurant, SmileyReport)> = Vec::new();

    let test_tuple_1 = (
        Restaurant {
            id: 1,
            smiley_restaurant_id: "1".to_string(),
            name: "Sej restaurant".to_string(),
            address: "Sejgade".to_string(),
            zipcode: 1337.to_string(),
            city: "Aalborg".to_string(),
            cvr: 9001.to_string(),
            pnr: 1234.to_string(),
            latitude: 1.0,
            longitude: 1.0,
            version_number: 1,
            region: Some("Nordjylland".to_string()),
            industry_code: 1.to_string(),
            industry_text: "This is a cool industry".to_string(),
            start_date: "1".to_string(),
            end_date: "".to_string(),
            elite_smiley: "Yes".to_string(),
            niche_industry: "No".to_string(),
            url: "www.myrestaurant.com".to_string(),
            ad_protection: "Yes".to_string(),
            company_type: "Restaurant".to_string(),
            franchise_name: Some("Best Franchise".to_string()),
        },
        SmileyReport {
            id: 1,
            res_id: 1,
            rating: 5,
            report_id: 1.to_string(),
            date: 11.to_string(),
        },
    );
    let test_tuple_2 = (
        Restaurant {
            id: 1,
            smiley_restaurant_id: "1".to_string(),
            name: "Sej restaurant".to_string(),
            address: "Sejgade".to_string(),
            zipcode: 1337.to_string(),
            city: "Aalborg".to_string(),
            cvr: 9001.to_string(),
            pnr: 1234.to_string(),
            latitude: 1.0,
            longitude: 1.0,
            version_number: 1,
            region: Some("Nordjylland".to_string()),
            industry_code: 1.to_string(),
            industry_text: "This is a cool industry".to_string(),
            start_date: "1".to_string(),
            end_date: "".to_string(),
            elite_smiley: "Yes".to_string(),
            niche_industry: "No".to_string(),
            url: "www.myrestaurant.com".to_string(),
            ad_protection: "Yes".to_string(),
            company_type: "Restaurant".to_string(),
            franchise_name: Some("Best Franchise".to_string()),
        },
        SmileyReport {
            id: 2,
            res_id: 1,
            rating: 4,
            report_id: 2.to_string(),
            date: 12.to_string(),
        },
    );

    let test_tuple_3 = (
        Restaurant {
            id: 2,
            smiley_restaurant_id: "2".to_string(),
            name: "Nedern restaurant".to_string(),
            address: "Nejgade".to_string(),
            zipcode: 1337.to_string(),
            city: "Aalborg".to_string(),
            cvr: 9001.to_string(),
            pnr: 1234.to_string(),
            latitude: 1.0,
            longitude: 1.0,
            version_number: 1,
            region: Some("Nordjylland".to_string()),
            industry_code: 1.to_string(),
            industry_text: "This is a cool industry".to_string(),
            start_date: "1".to_string(),
            end_date: "".to_string(),
            elite_smiley: "Yes".to_string(),
            niche_industry: "No".to_string(),
            url: "www.myrestaurant.com".to_string(),
            ad_protection: "Yes".to_string(),
            company_type: "Restaurant".to_string(),
            franchise_name: Some("Best Franchise".to_string()),
        },
        SmileyReport {
            id: 3,
            res_id: 2,
            rating: 4,
            report_id: 2.to_string(),
            date: 12.to_string(),
        },
    );

    vec.push(test_tuple_1);
    vec.push(test_tuple_2);
    vec.push(test_tuple_3);

    let res = conv_res_smiley_to_jsonres(vec);

    // Check if there exists only 2 json restaurants
    assert_eq!(res.len(), 2);
    // Check if first restaurant has 2 smiley reports, and second one has 1 smiley report
    assert_eq!(res.get(0).unwrap().smiley_reports.len(), 2);
    assert_eq!(res.get(1).unwrap().smiley_reports.len(), 1);
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
        end_date: (*restaurant.end_date).to_string(),
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

pub fn delete_smiley_data(json: &String, connection: &SqliteConnection) {
    let data_to_delete: DeleteData = serde_json::from_str(json).expect("Can't parse json");
    let ver = Version::get_from_token(connection, &data_to_delete.token);
    remove_restaurant(&connection, data_to_delete.data, &ver);
}
