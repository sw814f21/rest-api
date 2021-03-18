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
    pub published: bool
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
