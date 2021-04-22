use crate::database::models::{Restaurant, SmileyReport};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RestaurantWithSmileyReport {
    pub id: i32,
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
    pub smileyreports: Vec<SmileyReport>,
}

pub fn convert_res_smiley_pairs(
    mut input: Vec<(Restaurant, SmileyReport)>,
) -> Vec<RestaurantWithSmileyReport> {
    let mut result: Vec<RestaurantWithSmileyReport> = Vec::new();
    let mut current: Restaurant;
    let mut smileys: Vec<SmileyReport> = Vec::new();
    if !input.is_empty() {
        let first = input.pop().unwrap();
        current = first.0;
        smileys.append(vec![first.1].as_mut());
        for i in input {
            if !(current.id == i.0.id) {
                result.append(make_res_smiley(current, smileys).as_mut());
                smileys = Vec::new();
                current = i.0;
            }
            smileys.append(vec![i.1].as_mut());
        }
        result.append(make_res_smiley(current, smileys).as_mut());
    }
    result
}
fn make_res_smiley(
    current: Restaurant,
    smileys: Vec<SmileyReport>,
) -> Vec<RestaurantWithSmileyReport> {
    vec![RestaurantWithSmileyReport {
        id: current.id,
        smiley_restaurant_id: current.smiley_restaurant_id,
        name: current.name,
        address: current.address,
        zipcode: current.zipcode,
        city: current.city,
        cvr: current.cvr,
        pnr: current.pnr,
        latitude: current.latitude,
        longitude: current.longitude,
        version_number: current.version_number,
        smileyreports: smileys,
    }]
}
