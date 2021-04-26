use crate::database::models::Version;
use crate::utils::json_parser::RichData;
use crate::{
    database::schema,
    utils::data_inserter::{insert_restaurant, insert_smileys, update_restaurant, update_smileys},
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

pub fn get_data(conn: &SqliteConnection) -> Vec<(Restaurant, SmileyReport)> {
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
    for joined in &joined_smiley_report_restaurnt {
        let joined_restaurant = &joined.0;
        let joined_smiley_report = &joined.1;

        println!("Name of joined restaurant: {}", joined_restaurant.name);
        println!("Smiley rating: {}", joined_smiley_report.rating)
    }

    // Return the joined result
    joined_smiley_report_restaurnt
}
