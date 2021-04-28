use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonRestaurant {
    #[serde(rename(serialize = "city", deserialize = "city"))]
    pub city: String,

    #[serde(rename(serialize = "cvrnr", deserialize = "cvrnr"))]
    pub cvr: String,

    #[serde(rename(serialize = "geo_lat", deserialize = "geo_lat"))]
    pub latitude: String,

    #[serde(rename(serialize = "geo_lng", deserialize = "geo_lng"))]
    pub longitude: String,

    #[serde(rename(serialize = "pnr", deserialize = "pnr"))]
    pub pnr: String,

    #[serde(rename(serialize = "address", deserialize = "address"))]
    pub address: String,

    #[serde(rename(serialize = "zip_code", deserialize = "zip_code"))]
    pub zipcode: String,

    #[serde(rename(serialize = "name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "name_seq_nr", deserialize = "name_seq_nr"))]
    pub smiley_restaurant_id: String,

    #[serde(rename(serialize = "smiley_reports", deserialize = "smiley_reports"))]
    pub smiley_reports: Vec<JsonSmileyReport>,

    #[serde(rename(serialize = "region", deserialize = "region"))]
    pub region: Option<String>,

    #[serde(rename(serialize = "industry_code", deserialize = "industry_code"))]
    pub industry_code: String,

    #[serde(rename(serialize = "industry_text", deserialize = "industry_text"))]
    pub industry_text: String,

    #[serde(rename(serialize = "start_date", deserialize = "start_date"))]
    pub start_date: String,

    #[serde(rename(serialize = "elite_smiley", deserialize = "elite_smiley"))]
    pub elite_smiley: String,

    #[serde(rename(serialize = "niche_industry", deserialize = "niche_industry"))]
    pub niche_industry: String,

    #[serde(rename(serialize = "url", deserialize = "url"))]
    pub url: String,

    #[serde(rename(serialize = "ad_protection", deserialize = "ad_protection"))]
    pub ad_protection: String,

    #[serde(rename(serialize = "company_type", deserialize = "company_type"))]
    pub company_type: String,

    #[serde(rename(serialize = "franchise_name", deserialize = "franchise_name"))]
    pub franchise_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonSmileyReport {
    #[serde(rename(serialize = "date", deserialize = "date"))]
    pub date: String,

    #[serde(rename(serialize = "smiley", deserialize = "smiley"))]
    pub smiley: i32,

    #[serde(rename(serialize = "report_id", deserialize = "report_id"))]
    pub report_id: String,
}

#[derive(Deserialize)]
pub struct RichData {
    #[serde(alias = "timestamp")]
    pub token: String,
    #[serde(alias = "data")]
    pub data: Vec<JsonRestaurant>,
}

#[derive(Deserialize)]
pub struct DeleteData {
    #[serde(alias = "timestamp")]
    pub token: String,
    #[serde(alias = "data")]
    pub data: Vec<i32>,
}
