use crate::database::establish_connection;
use crate::database::restaurants_repository;
use crate::database::schema::restaurants;
use crate::database::schema::smileyreports;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::{borrow::Borrow, fs::File};

#[derive(Deserialize, Serialize)]
pub struct ParseRestaurants {
    #[serde(alias = "By")]
    pub city: String,

    #[serde(alias = "cvrnr")]
    pub cvr: String,

    #[serde(alias = "Geo_Lat")]
    pub latitude: String,

    #[serde(alias = "Geo_Lng")]
    pub longitude: String,

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
    #[serde(alias = "date")]
    pub date: String,

    #[serde(alias = "smiley")]
    pub rating: String,

    #[serde(alias = "report_id")]
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
    pub restaurant_id: i32,
    pub date: String,
    pub rating: i32,
    pub report_id: String,
}

pub fn load_data(path: &String) {
    let file = File::open(path).expect("Can't open file from path");
    let reader = BufReader::new(file);
    let restaurants_vector: Vec<ParseRestaurants> =
        serde_json::from_reader(reader).expect("Can't parse json");

    let mut newsmileyreports: Vec<NewSmileyReport> = Vec::new();

    let connection_pool = establish_connection();
    let connection = connection_pool.get().expect("Can't get connection");

    for res in restaurants_vector {
        let resid = restaurants_repository::insert_restaurants(
            &connection,
            NewRestaurant {
                city: res.city,
                cvr: res.cvr,
                latitude: res.latitude.parse::<f32>().unwrap(),
                longitude: res.longitude.parse::<f32>().unwrap(),
                pnr: res.pnr.to_string(),
                address: res.address,
                url: res.url,
                zipcode: res.zipcode,
                name: res.name,
            },
        );
        match res.latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x, resid)),
        }
        match res.second_latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x, resid)),
        }
        match res.third_latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x, resid)),
        }
        match res.fourth_latest_control {
            None => {}
            Some(x) => newsmileyreports.push(extractsmiley(x, resid)),
        }
    }

    restaurants_repository::insert_smileys(&connection, &newsmileyreports);

    println!("Finished loading data into database")
}

pub fn extractsmiley(input: ParseSmileyReports, resid: i32) -> NewSmileyReport {
    NewSmileyReport {
        restaurant_id: resid,
        date: convertdate(input.date),
        rating: input.rating.parse::<i32>().unwrap(),
        report_id: input.report_id,
    }
}

pub fn convertdate(input: String) -> String {
    let dateandtime: Vec<&str> = input.split(" ").collect();
    let date: Vec<&str> = match dateandtime.get(0) {
        None => {
            vec![""]
        }
        Some(x) => x.split("-").collect(),
    };
    let mut output = String::new();
    for i in date {
        if output.is_empty() {
            output = i.to_owned() + " ";
        } else {
            output = i.to_owned() + "-" + output.borrow();
        }
    }
    output
        + match dateandtime.get(1) {
            None => " ",
            Some(x) => x.to_owned(),
        }
}
