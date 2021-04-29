use std::borrow::Borrow;

use crate::database::append_smiley::RestaurantWithSmileyReport;
use crate::database::models::Restaurant;
use actix_web::{get, web, HttpResponse, Responder};
use array_tool;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::Deserialize;

#[get("/restaurant")]
pub async fn restaurant(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Restaurant::get_all_resturants(&conn))
}

#[get("/restaurant/{id}")]
pub async fn restaurant_by_id(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Restaurant::get_restaurant_by_id(id, &conn))
}

#[derive(Deserialize)]
pub struct Restaurantsearchinput {
    id: Option<i32>,
    name: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    location: Option<String>, /*xx.yy,xx.yy,xx.yy,xx.yy*/
}

use array_tool::vec::Intersect;
#[get("/restaurant/search")]
pub async fn search_restaurants(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    input: web::Query<Restaurantsearchinput>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let mut idsearch: Vec<RestaurantWithSmileyReport> = Vec::new();
    let mut namesearch: Vec<RestaurantWithSmileyReport> = Vec::new();
    let mut citysearch: Vec<RestaurantWithSmileyReport> = Vec::new();
    let mut zipsearch: Vec<RestaurantWithSmileyReport> = Vec::new();
    let mut locationsearch: Vec<RestaurantWithSmileyReport> = Vec::new();
    let mut queryoutput: Vec<RestaurantWithSmileyReport> = Vec::new();

    match input.id {
        None => {}
        Some(x) => {
            idsearch.append(vec![Restaurant::get_restaurant_by_id(x, &conn)].as_mut());
        }
    }
    match input.name.borrow() {
        None => {}
        Some(x) => {
            namesearch.append(Restaurant::search_by_name(x.to_string(), &conn).as_mut());
        }
    }
    match input.zip.borrow() {
        None => {}
        Some(x) => {
            zipsearch.append(Restaurant::search_by_zip(x.to_string(), &conn).as_mut());
        }
    }
    match input.city.borrow() {
        None => {}
        Some(x) => {
            citysearch.append(Restaurant::search_by_city(x.to_string(), &conn).as_mut());
        }
    }
    match input.location.borrow() {
        None => {}
        Some(x) => {
            let mut strcords = x.split(",");
            let nwlat = strcords.next().unwrap().parse::<f64>();
            let nwlng = strcords.next().unwrap().parse::<f64>();
            let selat = strcords.next().unwrap().parse::<f64>();
            let selng = strcords.next().unwrap().parse::<f64>();
            if nwlat.is_ok() && nwlng.is_ok() && selat.is_ok() && selng.is_ok() {
                locationsearch.append(
                    Restaurant::search_by_lat_lng(
                        nwlat.unwrap(),
                        nwlng.unwrap(),
                        selat.unwrap(),
                        selng.unwrap(),
                        &conn,
                    )
                    .as_mut(),
                );
            }
        }
    }

    let results = vec![idsearch, namesearch, citysearch, zipsearch, locationsearch];
    for r in results {
        if queryoutput.is_empty() {
            queryoutput = r.to_vec();
        }
        if !r.is_empty() {
            queryoutput = queryoutput.intersect(r);
        }
    }
    HttpResponse::Ok().json(queryoutput)
}
