use crate::database::schema::restaurant;
use crate::database::schema::smiley_report;
use diesel::prelude::*;

#[derive(Insertable)]
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
}

#[derive(Insertable)]
#[table_name = "smiley_report"]
pub struct InsertSmileyReport {
    pub date: String,
    pub smiley: i32,
    pub restaurant_id: i32,
    pub report_id: String,
}

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub fn insert_restaurant(conn: &SqliteConnection, restaurants_data: &InsertRestaurant) -> i32 {
    diesel::insert_into(restaurant::table)
        .values(restaurants_data)
        .execute(conn)
        .expect("Error saving new restaurant");
    
    diesel::select(last_insert_rowid).get_result::<i32>(conn).expect("Error getting result")
}

pub fn insert_smileys(conn: &SqliteConnection, smiley_data: &Vec<InsertSmileyReport>) -> usize {
    diesel::insert_into(smiley_report::table)
        .values(smiley_data)
        .execute(conn)
        .expect("Error saving new smiley data")
}