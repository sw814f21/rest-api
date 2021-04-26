use crate::database::models::{Restaurant, SmileyReport, Version};
use crate::database::schema::{removed_restaurant, restaurant, smiley_report};
use crate::utils::json_parser::{JsonRestaurant, JsonSmileyReport};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset)]
#[table_name = "restaurant"]
pub struct InsertRestaurant {
    pub smiley_restaurant_id: i32,
    pub name: String,
    pub address: String,
    pub zipcode: String,
    pub city: String,
    pub cvr: String,
    pub pnr: String,
    pub latitude: f32,
    pub longitude: f32,
    pub version_number: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "smiley_report"]
pub struct InsertSmileyReport {
    pub date: String,
    pub smiley: i32,
    pub restaurant_id: i32,
    pub report_id: String,
}

#[derive(Insertable)]
#[table_name = "removed_restaurant"]
pub struct InsertRemovedRestaurant {
    pub restaurant_id: i32,
    pub version_number: i32,
}

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub fn insert_restaurant(
    conn: &SqliteConnection,
    restaurants_data: &JsonRestaurant,
    version: i32,
) -> i32 {
    let insertable = map_restaurant_json2insert(restaurants_data, version);

    diesel::insert_into(restaurant::table)
        .values(&insertable)
        .execute(conn)
        .expect("Error saving new restaurant");

    diesel::select(last_insert_rowid)
        .get_result::<i32>(conn)
        .expect("Error getting result")
}

pub fn insert_smileys(
    conn: &SqliteConnection,
    smiley_data: &Vec<JsonSmileyReport>,
    restaurant_id: i32,
) -> usize {
    let mut new_smileyreports = Vec::new();
    for report in smiley_data {
        new_smileyreports.push(map_smileyreport_json2insert(&report, restaurant_id));
    }

    diesel::insert_into(smiley_report::table)
        .values(new_smileyreports)
        .execute(conn)
        .expect("Error saving new smiley data")
}

pub fn remove_restaurant(conn: &SqliteConnection, restaurant_ids: Vec<i32>, version: &Version) {
    let removed_restaurant_entry: Vec<InsertRemovedRestaurant> = restaurant_ids
        .iter()
        .map(|id| InsertRemovedRestaurant {
            restaurant_id: *id,
            version_number: version.id,
        })
        .collect();

    diesel::insert_into(removed_restaurant::table)
        .values(&removed_restaurant_entry)
        .execute(conn)
        .expect("Failed to add removed restaurant entry");

    diesel::delete(restaurant::table)
        .filter(restaurant::id.eq_any(&restaurant_ids))
        .execute(conn)
        .expect("Failed to delete restaurant entry");
}

pub fn update_restaurant(
    conn: &SqliteConnection,
    restaurant: &JsonRestaurant,
    version: i32,
) -> i32 {
    let insertable_restaurant = map_restaurant_json2insert(restaurant, version);

    diesel::update(restaurant::table)
        .filter(restaurant::smiley_restaurant_id.eq(insertable_restaurant.smiley_restaurant_id))
        .set(&insertable_restaurant)
        .execute(conn)
        .expect("Failed to update restaurant");
    Restaurant::get_restaurant_by_smiley_id(insertable_restaurant.smiley_restaurant_id, conn)
}

pub fn update_smileys(conn: &SqliteConnection, smiley_data: &JsonSmileyReport, restaurant_id: i32) {
    let exists = SmileyReport::smiley_report_exists(restaurant_id, &smiley_data.report_id, conn);

    if exists {
        diesel::update(smiley_report::table)
            .filter(smiley_report::report_id.eq(&smiley_data.report_id))
            .set((
                smiley_report::smiley.eq(smiley_data.smiley),
                smiley_report::date.eq(&smiley_data.date),
            ))
            .execute(conn)
            .expect("Failed to update smiley report");
    } else {
        let insert_data = map_smileyreport_json2insert(smiley_data, restaurant_id);

        diesel::insert_into(smiley_report::table)
            .values(insert_data)
            .execute(conn)
            .expect("Error saving new smiley data");
    }
}

fn map_restaurant_json2insert(input: &JsonRestaurant, version_number: i32) -> InsertRestaurant {
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
        version_number: version_number,
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
