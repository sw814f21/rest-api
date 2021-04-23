use crate::database::models::Version;
use crate::utils::json_parser::JsonRestaurant;
use crate::{
    database::schema,
    utils::data_inserter::{insert_restaurant, insert_smileys},
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
    let read_json: Vec<JsonRestaurant> = serde_json::from_str(json).expect("Can't parse json");

    let new_version = Version::create_new_version(connection);

    for res in read_json {
        let resid = insert_restaurant(&connection, &res, &new_version);

        insert_smileys(&connection, &res.smiley_reports, resid);
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
