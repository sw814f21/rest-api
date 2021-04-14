use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Simplerestaurant {
    #[serde(alias = "id")]
    pub id: i32,
    #[serde(alias = "lat")]
    pub lat: f32,
    #[serde(alias = "lng")]
    pub lng: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Restaurant {
    #[serde(alias = "id")]
    pub id: i32,

    #[serde(alias = "smiley_restaurant_id")]
    pub smiley_restaurant_id: i32,

    #[serde(alias = "name")]
    pub name: String,

    #[serde(alias = "address")]
    pub address: String,

    #[serde(alias = "zipcode")]
    pub zipcode: String,

    #[serde(alias = "city")]
    pub city: String,

    #[serde(alias = "cvr")]
    pub cvr: String,

    #[serde(alias = "pnr")]
    pub pnr: String,

    #[serde(alias = "latitude")]
    pub latitude: f32,

    #[serde(alias = "longitude")]
    pub longitude: f32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SmileyReport {
    #[serde(alias = "id")]
    pub id: i32,

    #[serde(alias = "res_id")]
    pub res_id: i32,

    #[serde(alias = "rating")]
    pub rating: i32,

    #[serde(alias = "date")]
    pub date: String,

    #[serde(alias = "report_id")]
    pub report_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Restaurantandsmiley {
    #[serde(alias = "id")]
    pub id: i32,

    #[serde(alias = "smiley_restaurant_id")]
    pub smiley_restaurant_id: i32,

    #[serde(alias = "name")]
    pub name: String,

    #[serde(alias = "address")]
    pub address: String,

    #[serde(alias = "zipcode")]
    pub zipcode: String,

    #[serde(alias = "city")]
    pub city: String,

    #[serde(alias = "cvr")]
    pub cvr: String,

    #[serde(alias = "pnr")]
    pub pnr: String,

    #[serde(alias = "latitude")]
    pub latitude: f32,

    #[serde(alias = "longitude")]
    pub longitude: f32,

    #[serde(alias = "smiley_reports")]
    pub smiley_reports: Vec<SmileyReport>,
}
