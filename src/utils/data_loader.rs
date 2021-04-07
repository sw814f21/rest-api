use crate::database::establish_connection;
use crate::database::restaurants_repository;
use crate::database::schema::restaurants;
use crate::database::schema::smileyreports;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Serialize)]
pub struct ParseRestaurants {
    #[serde(alias = "By")]
    pub city: String,

    #[serde(alias = "cvrnr")]
    pub cvr: String,

    #[serde(alias = "Geo_Lat")]
    pub latitude: f32,

    #[serde(alias = "Geo_Lng")]
    pub longitude: f32,

    #[serde(alias = "pnr")]
    pub pnr: String,

    #[serde(alias = "adresse1")]
    pub address: String,

    #[serde(alias = "URL")]
    pub url: String,

    #[serde(alias = "postnr")]
    pub zipcode: String,

    #[serde(alias = "navn1")]
    pub name: String,

    #[serde(alias = "seneste_kontrol")]
    pub latest_control: Option<ParseSmileyReports>,

    #[serde(alias = "naestseneste_kontrol")]
    pub second_latest_control: Option<ParseSmileyReports>,

    #[serde(alias = "tredjeseneste_kontrol")]
    pub third_latest_control: Option<ParseSmileyReports>,

    #[serde(alias = "fjerdeseneste_kontrol")]
    pub fourth_latest_control: Option<ParseSmileyReports>,
}
#[derive(Deserialize, Serialize)]
pub struct ParseSmileyReports {
    #[serde(alias = "pnr name")]
    pub pnr: String,

    #[serde(alias = "date name")]
    pub date: String,

    #[serde(alias = "rating name")]
    pub rating: i32,

    #[serde(alias = "report name")]
    pub report_id: String,
}
#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "restaurants"]
pub struct NewRestaurant {
    pub city: String,
    pub cvr: String,
    pub latitude: f32,
    pub longitude: f32,
    pub pnr: String,
    pub address: String,
    pub url: String,
    pub zipcode: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "smileyreports"]
pub struct NewSmileyReport {
    pub pnr: String,
    pub date: String,
    pub rating: i32,
    pub report_id: String,
}

pub fn load_data(path: &String) {
    let file = File::open(path).expect("Can't open file from path");
    let reader = BufReader::new(file);
    let restaurants_vector: Vec<ParseRestaurants> =
        serde_json::from_reader(reader).expect("Can't parse json");

    let mut newrestaurants: Vec<NewRestaurant> = Vec::new();
    let mut newsmileyreports: Vec<NewSmileyReport> = Vec::new();

    for res in restaurants_vector {
        newrestaurants.push(NewRestaurant {
            city: res.city,
            cvr: res.cvr,
            latitude: res.latitude,
            longitude: res.longitude,
            pnr: res.pnr,
            address: res.address,
            url: res.url,
            zipcode: res.zipcode,
            name: res.name,
        });
        match res.latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x)),
        }
        match res.second_latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x)),
        }
        match res.third_latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x)),
        }
        match res.fourth_latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x)),
        }
    }

    let connection_pool = establish_connection();
    let connection = connection_pool.get().expect("Can't get connection");

    restaurants_repository::insert_restaurants(&connection, &newrestaurants);
    restaurants_repository::insert_smileys(&connection, &newsmileyreports);

    println!("Finished loading data into database")
}

pub fn extractsmiley(input: ParseSmileyReports) -> NewSmileyReport {
    NewSmileyReport {
        pnr: input.pnr,
        date: input.date,
        rating: input.rating,
        report_id: input.report_id,
    }
}
