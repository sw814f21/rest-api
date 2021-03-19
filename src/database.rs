use dotenv::dotenv;
use diesel::{r2d2::{self, ConnectionManager, Pool}, sqlite::SqliteConnection};
use std::env;

pub mod models;
pub mod schema;
pub mod restaurants_repository;

embed_migrations!();

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;


pub fn run_migrations(conn: &SqliteConnection) {
  let _ = diesel_migrations::run_pending_migrations(&*conn);
}

pub fn establish_connection() -> DbPool {
    if cfg!(test) {
        let manager = ConnectionManager::<SqliteConnection>::new("file::memory:?cache=shared");
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB pool.");
        run_migrations(&pool.get().unwrap());

        pool
    } else {
        dotenv().ok();
    
        let database_url = env::var("DatabaseFile").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
        
        r2d2::Pool::builder().build(manager).expect("Failed to create DB pool.")
    }
}
