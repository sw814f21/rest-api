use diesel::{
    r2d2::{self, ConnectionManager, Pool},
    sqlite::SqliteConnection,
};
use std::env;

pub mod models;
pub mod schema;

embed_migrations!();

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn run_migrations(conn: &SqliteConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

pub fn new_pool() -> DbPool {
    if cfg!(test) {
        println!("Creating in-memory db");
        let manager = ConnectionManager::<SqliteConnection>::new("file::memory:");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create DB pool.");
        run_migrations(&pool.get().unwrap());

        pool
    } else {
        match env::var("DATABASE_URL") {
            Ok(database_url) => {
                let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

                r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create DB pool.")
            }
            Err(_) => {
                panic!("DATABASE_URL is not set in .env file");
            }
        }
    }
}
