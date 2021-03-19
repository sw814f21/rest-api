use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use super::schema::posts;
use super::schema::restaurants;
use super::schema::posts::dsl::posts as post_dsl;


#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Queryable)]
pub struct Restaurant {
    pub id: i32,
    pub city: String,
    pub cvr: String,
    pub longitude: f32,
    pub latitude: f32,
    pub pnr: String,
    pub address: String,
    pub url: String,
    pub zipcode: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "restaurants"]
pub struct NewRestaurant {
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
}

impl Post {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        post_dsl.load::<Post>(conn).expect("Error loading posts")
    }

    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        post_dsl.find(id).get_result::<Post>(conn).ok()
    }
    /*
    pub fn by_email(email_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::users::dsl::email;

        user_dsl.filter(email.eq(email_str)).first::<Post>(conn).ok()
    }

    pub fn by_phone(phone_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::users::dsl::phone;

        user_dsl.filter(phone.eq(phone_str)).first::<Post>(conn).ok()
    }*/

    /*pub fn create(email: Option<&str>, phone: Option<&str>, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().to_hyphenated().to_string();
        
        if email.is_none() && phone.is_none() {
            return None
        } 
                
        if phone.is_some() {
            if let Some(user) = Self::by_phone(&phone.unwrap(), conn) {
                return Some(user)
            } 
        }
        
        if email.is_some() {
            if let Some(user) = Self::by_email(&email.unwrap(), conn) {
                return Some(user)
            } 
        }

        let new_user = Self::new_user_struct(&new_id, phone, email);

        diesel::insert_into(user_dsl)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");

        Self::by_id(&new_id, conn)
    }*/

    /*fn new_user_struct(id: i32, phone: Option<&str>, email: Option<&str>, published: bool) -> Self {
        Post {
            id: id.into(),
            title: title.map(Into::into),
            body: body.map(Into::into),
            published: published.into()
        }
    }*/
}
