use diesel::prelude::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use serde::{Deserialize, Serialize};

use super::schema::posts;
use super::schema::posts::dsl::posts as user_dsl;


#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "posts"]
pub struct User {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

impl User {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        user_dsl.load::<User>(conn).expect("Error loading posts")
    }

    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        user_dsl.find(id).get_result::<User>(conn).ok()
    }
    /*
    pub fn by_email(email_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::users::dsl::email;

        user_dsl.filter(email.eq(email_str)).first::<User>(conn).ok()
    }

    pub fn by_phone(phone_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::users::dsl::phone;

        user_dsl.filter(phone.eq(phone_str)).first::<User>(conn).ok()
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
        User {
            id: id.into(),
            title: title.map(Into::into),
            body: body.map(Into::into),
            published: published.into()
        }
    }*/
}