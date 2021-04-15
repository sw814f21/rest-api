use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonRestaurant {
    #[serde(alias = "city")]
    pub city: String,

    #[serde(alias = "cvrnr")]
    pub cvr: String,

    #[serde(alias = "geo_lat")]
    pub latitude: f32,

    #[serde(alias = "geo_lng")]
    pub longitude: f32,

    #[serde(alias = "pnr")]
    pub pnr: String,

    #[serde(alias = "adresse1")]
    pub address: String,

    #[serde(alias = "zip_code")]
    pub zipcode: String,

    #[serde(alias = "navn1")]
    pub name: String,

    #[serde(alias = "name_seq_nr")]
    pub smiley_restaurant_id: String,

    #[serde(alias = "smiley_reports")]
    pub smiley_reports: Vec<JsonSmileyReport>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonSmileyReport {
    #[serde(alias = "date")]
    pub date: String,

    #[serde(alias = "smiley")]
    pub smiley: i32,

    #[serde(alias = "report_id")]
    pub report_id: String,
}
