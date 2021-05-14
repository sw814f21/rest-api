use crate::database::models::{Restaurant, SmileyReport, Version};
use crate::database::schema::{removed_restaurant, restaurant, smiley_report};
use crate::utils::json_parser::{JsonRestaurant, JsonSmileyReport};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset)]
#[table_name = "restaurant"]
pub struct InsertRestaurant {
    pub smiley_restaurant_id: String,
    pub name: String,
    pub address: String,
    pub zipcode: String,
    pub city: String,
    pub cvr: String,
    pub pnr: String,
    pub latitude: f64,
    pub longitude: f64,
    pub version_number: i32,
    pub region: Option<String>,
    pub industry_code: String,
    pub industry_text: String,
    pub start_date: String,
    pub end_date: String,
    pub elite_smiley: String,
    pub niche_industry: String,
    pub url: String,
    pub ad_protection: String,
    pub company_type: String,
    pub franchise_name: Option<String>,
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

pub fn insert_restaurants(
    conn: &SqliteConnection,
    restaurants_data: &Vec<InsertRestaurant>,
) -> Vec<(String, i32)> {
    diesel::insert_into(restaurant::table)
        .values(restaurants_data)
        .execute(conn)
        .expect("Error saving new restaurant");

    restaurant::table
        .select((restaurant::smiley_restaurant_id, restaurant::id))
        .get_results::<(String, i32)>(conn)
        .expect("Unable to fetch ids from the")
}

pub fn insert_smileys(conn: &SqliteConnection, smiley_data: &Vec<InsertSmileyReport>) {
    diesel::insert_into(smiley_report::table)
        .values(smiley_data)
        .execute(conn)
        .expect("Error saving new smiley data");
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
        .filter(restaurant::smiley_restaurant_id.eq(&insertable_restaurant.smiley_restaurant_id))
        .set(&insertable_restaurant)
        .execute(conn)
        .expect("Failed to update restaurant");
    Restaurant::get_restaurant_by_smiley_id(&insertable_restaurant.smiley_restaurant_id, conn)
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

pub(crate) fn map_restaurant_json2insert(
    input: &JsonRestaurant,
    version_number: i32,
) -> InsertRestaurant {
    InsertRestaurant {
        smiley_restaurant_id: (*input.smiley_restaurant_id).to_string(),
        name: (*input.name).to_string(),
        address: (*input.address).to_string(),
        zipcode: (*input.zipcode).to_string(),
        city: (*input.city).to_string(),
        cvr: (*input.cvr).to_string(),
        pnr: (*input.pnr).to_string(),
        latitude: input.latitude,
        longitude: input.longitude,
        version_number: version_number,
        region: input.region.clone(),
        industry_code: (*input.industry_code).to_string(),
        industry_text: (*input.industry_text).to_string(),
        start_date: (*input.start_date).to_string(),
        end_date: (*input.end_date).to_string(),
        elite_smiley: (*input.elite_smiley).to_string(),
        niche_industry: (*input.niche_industry).to_string(),
        url: (*input.url).to_string(),
        ad_protection: (*input.ad_protection).to_string(),
        company_type: (*input.company_type).to_string(),
        franchise_name: input.franchise_name.clone(),
    }
}

pub fn map_smileyreport_json2insert(input: &JsonSmileyReport, res_id: i32) -> InsertSmileyReport {
    InsertSmileyReport {
        date: (*input.date).to_string(),
        smiley: input.smiley,
        restaurant_id: res_id,
        report_id: (*input.report_id).to_string(),
    }
}
