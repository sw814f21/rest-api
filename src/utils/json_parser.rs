use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonRestaurant {
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

    #[serde(alias = "postnr")]
    pub zipcode: String,

    #[serde(alias = "navn1")]
    pub name: String,

    #[serde(alias = "navnelbnr")]
    pub smiley_restaurant_id: i32,

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