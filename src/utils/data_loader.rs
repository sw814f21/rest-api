use super::json_parser::DeleteJson;
use crate::database::models::Version;
use crate::utils::data_inserter::{insert_restaurant, insert_smileys, remove_restaurant};
use crate::utils::json_parser::JsonRestaurant;
use diesel::SqliteConnection;

pub fn load_data_from_file(path: &String, conn: &SqliteConnection) {
    let json = std::fs::read_to_string(path).expect("Failed to read file");

    load_data(&json, conn);

    println!("Finished loading data into database");
}

pub fn load_data(json: &String, connection: &SqliteConnection) {
    let read_json: Vec<JsonRestaurant> = serde_json::from_str(json).expect("Can't parse json");

    let new_version = Version::create_new_version(connection);

    for res in read_json {
        let resid = insert_restaurant(&connection, &res, &new_version);

        insert_smileys(&connection, &res.smiley_reports, resid);
    }
}

pub fn delete_smiley_records(json: &String, connection: &SqliteConnection) {
    let data_to_delete: DeleteJson = serde_json::from_str(json).expect("Can't parse json");
    let version = Version::version_from_str(connection, &data_to_delete.timestamp);
    remove_restaurant(&connection, data_to_delete.data, &version);
}
