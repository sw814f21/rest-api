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
    pub name: String,
    pub latest_control: Option<i32>,
    pub second_latest_control: Option<i32>,
    pub third_latest_control: Option<i32>,
    pub fourth_latest_control: Option<i32>,
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

    #[serde(alias = "navn1")]
    pub name: String,

    #[serde(alias = "seneste_kontrol")]
    pub latest_control: Option<i32>,

    #[serde(alias = "naestseneste_kontrol")]
    pub second_latest_control: Option<i32>,

    #[serde(alias = "tredjeseneste_kontrol")]
    pub third_latest_control: Option<i32>,

    #[serde(alias = "fjerdeseneste_kontrol")]
    pub fourth_latest_control: Option<i32>,
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

    pub fn create(conn: &SqliteConnection, post: Post) -> Option<Self> {
        let id = post.id;

        diesel::insert_into(post_dsl)
            .values(&post)
            .execute(conn)
            .expect("Error saving new user");

        Self::by_id(id, conn)
    }

    /*fn new_user_struct(id: i32, phone: Option<&str>, email: Option<&str>, published: bool) -> Self {
        Post {
            id: id.into(),
            title: title.map(Into::into),
            body: body.map(Into::into),
            published: published.into()
        }
    }*/
}
