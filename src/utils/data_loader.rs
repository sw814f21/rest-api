use crate::utils::json_parser::JsonRestaurant;
use crate::{database::models::Version, services::restaurant::restaurant};
use crate::{
    database::schema,
    utils::data_inserter::{insert_restaurant, insert_smileys},
};
use diesel::{JoinOnDsl, QueryDsl, SqliteConnection};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use diesel::dsl::{delete, exists, insert_into, select};

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

pub fn get_data(conn: &SqliteConnection) {
    use schema::*;

    //let implicit_on_clause = smiley_report::table.inner_join(restaurant::table);

    let test = restaurant::table
        .inner_join(smiley_report::table.on(smiley_report::restaurant_id.eq(restaurant::id)))
        .load::<(Restaurant, SmileyReport)>(conn);

    //let test = smiley_report::table
    //.inner_join(restaurant::table.on(restaurant::id.eq(smiley_report::restaurant_id)));

    /*smiley_report::table
    .filter(restaurant::version_number.gt(version))
    .load::<Restaurant>(conn)
    .expect("Failed to get restaurants based on version")*/
    // TODO: We should read the restaurant and smiley_report, and join on the restaurant_id. The result should be delivered as an output.
}
