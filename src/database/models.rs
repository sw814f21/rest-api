use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::posts;
use super::schema::posts::dsl::posts as post_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Clone, PartialEq, Queryable, Serialize)]
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
}

#[derive(Clone, PartialEq, Queryable, Serialize)]
pub struct SmileyReport {
    pub id: i32,
    pub pnr: String,
    pub rating: i32,
    pub date: String,
    pub report_id: String,
}
#[derive(Queryable, Deserialize, Serialize)]
pub struct Simplerestaurant {
    id: i32,
    lat: f32,
    lng: f32,
}

use super::schema::restaurants::dsl::restaurants as res_dsl;
impl Restaurant {
    pub fn get_all_resturants(conn: &SqliteConnection) -> Vec<Simplerestaurant> {
        use super::schema::restaurants::dsl::id;
        use super::schema::restaurants::dsl::latitude;
        use super::schema::restaurants::dsl::longitude;
        res_dsl
            .select((id, latitude, longitude))
            .load::<Simplerestaurant>(conn)
            .expect("Error fetching restaurant data")
    }
    pub fn get_restaurant_by_id(res_id: i32, conn: &SqliteConnection) -> Restaurant {
        res_dsl
            .find(res_id)
            .get_result::<Restaurant>(conn)
            .ok()
            .expect("Error fetching restaurant ID")
    }

    pub fn search_by_lat_lng(
        nwlat: f32,
        nwlng: f32,
        selat: f32,
        selng: f32,
        conn: &SqliteConnection,
    ) -> Vec<Restaurant> {
        use super::schema::restaurants::dsl::latitude;
        use super::schema::restaurants::dsl::longitude;
        res_dsl
            .filter(latitude.lt(nwlat))
            .filter(latitude.gt(selat))
            .filter(longitude.gt(nwlng))
            .filter(longitude.lt(selng))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error fetching with Latitude/Longitude")
    }

    pub fn search_by_name(query: String, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::restaurants::dsl::name;
        res_dsl
            .filter(name.like(query + "%"))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error searching with restaurant name")
    }

    pub fn search_by_zip(query: String, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::restaurants::dsl::zipcode;
        res_dsl
            .filter(zipcode.eq(query))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error searching for restaurants with zipcode")
    }

    pub fn search_by_city(query: String, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::restaurants::dsl::city;
        res_dsl
            .filter(city.like(query + "%"))
            .get_results::<Restaurant>(conn)
            .ok()
            .expect("Error searching for restaurants with city")
    }
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

use super::schema::favorites;
use super::schema::favorites::dsl::favorites as fav_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "favorites"]
pub struct Favorites {
    pub restaurant_id: i32,
    pub token_id: String,
}

impl Favorites {
    pub fn add_favorite(res_id: i32, u_id: String, conn: &SqliteConnection) -> Self {
        match Favorites::find_favorite(res_id, u_id.to_string(), conn) {
            None => {
                match u_dsl.find(u_id.to_string()).first::<User>(conn).ok() {
                    None => {
                        User::new_user(u_id.to_string(), &conn);
                    }
                    Some(_) => {}
                }
                let new_fav = Favorites {
                    restaurant_id: res_id,
                    token_id: u_id.to_string(),
                };
                diesel::insert_into(fav_dsl)
                    .values(&new_fav)
                    .execute(conn)
                    .expect("error saving favorite");
                new_fav
            }
            Some(fav) => fav,
        }
    }

    pub fn remove_favorite(res_id: i32, u_id: String, conn: &SqliteConnection) {
        match Favorites::find_favorite(res_id, u_id, conn) {
            None => {}
            Some(fav) => {
                diesel::delete(fav_dsl.find((fav.restaurant_id, fav.token_id)))
                    .execute(conn)
                    .expect("error deleting");
            }
        }
    }

    pub fn find_favorite(res_id: i32, u_id: String, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::favorites::dsl::restaurant_id;
        use super::schema::favorites::dsl::token_id;

        fav_dsl
            .filter(restaurant_id.eq(res_id))
            .filter(token_id.eq(u_id))
            .first::<Favorites>(conn)
            .ok()
    }
    pub fn user_favorites(u_id: String, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::favorites::dsl::token_id;
        fav_dsl
            .filter(token_id.eq(u_id))
            .get_results::<Favorites>(conn)
            .expect("Error loading posts")
    }
}

use super::schema::users;
use super::schema::users::dsl::users as u_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub token_id: String,
    pub notifications: i32,
}

impl User {
    pub fn new_user(id: String, conn: &SqliteConnection) {
        let a_user = User {
            token_id: id,
            notifications: 0,
        };
        match u_dsl.find(&a_user.token_id).first::<User>(conn).ok() {
            None => {
                diesel::insert_into(u_dsl)
                    .values(&a_user)
                    .execute(conn)
                    .expect("failed to insert new user");
            }
            Some(_) => {}
        }
    }

    pub fn notification_change(id: String, conn: &SqliteConnection) {
        use super::schema::users::dsl::notifications;
        use super::schema::users::dsl::token_id;
        let a_user = u_dsl.find(id).first::<User>(conn).ok();
        match a_user {
            None => {}
            Some(x) => {
                diesel::update(u_dsl.filter(token_id.eq(x.token_id)))
                    .set(notifications.eq(if x.notifications == 1 { 0 } else { 1 }))
                    .execute(conn)
                    .expect("error updating notification setting");
            }
        }
    }

    pub fn remove_user(id: String, conn: &SqliteConnection) {
        use super::schema::users::dsl::token_id;
        diesel::delete(u_dsl.filter(token_id.eq(id)))
            .execute(conn)
            .expect("error removing user");
    }
}
