//cfg(test) tells compiler only to use this when running tests
#[cfg(test)]
mod tests {
    use crate::database::schema::*;
    use crate::utils::data_inserter::*;
    use crate::{database::models::*, utils::data_loader::load_data};
    use actix_web::http::Method;
    use actix_web::{
        test::{init_service, read_body, TestRequest},
        web, App,
    };
    use diesel::prelude::*;

    use crate::diesel::connection::SimpleConnection;
    use diesel::r2d2::CustomizeConnection;
    use diesel::{
        dsl::{delete, exists, insert_into, select},
        r2d2::PooledConnection,
    };
    use diesel::{
        r2d2::{self, ConnectionManager, Pool},
        sqlite::SqliteConnection,
        QueryDsl, RunQueryDsl,
    };
    use diesel_migrations;
    use std::time::Duration;

    #[derive(Debug)]
    pub struct ConnectionOptions {
        pub enable_wal: bool,
        pub enable_foreign_keys: bool,
        pub busy_timeout: Option<Duration>,
    }

    impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
        for ConnectionOptions
    {
        fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
            (|| {
                if self.enable_wal {
                    conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
                }
                if self.enable_foreign_keys {
                    conn.batch_execute("PRAGMA foreign_keys = ON;")?;
                }
                if let Some(d) = self.busy_timeout {
                    conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
                }
                Ok(())
            })()
            .map_err(diesel::r2d2::Error::QueryError)
        }
    }

    async fn init_database() -> Pool<ConnectionManager<SqliteConnection>> {
        let pool = {
            let manager = ConnectionManager::<SqliteConnection>::new("test_db.db");
            r2d2::Pool::builder()
                .connection_customizer(Box::new(ConnectionOptions {
                    enable_wal: true,
                    enable_foreign_keys: true,
                    busy_timeout: Some(Duration::from_secs(15)),
                }))
                .build(manager)
                .expect("error making db in test")
        };
        let _ = diesel_migrations::run_pending_migrations(&pool.get().unwrap());
        pool
    }

    async fn load_test_data(conn: &SqliteConnection) {
        load_data(&String::from("test_sample_data.json"), conn);
    }

    async fn clear_database(conn: &SqliteConnection) {
        let _ = diesel::delete(restaurant::table).execute(conn);
        let _ = diesel::delete(notification_history::table).execute(conn);
        let _ = diesel::delete(smiley_report::table).execute(conn);
        let _ = diesel::delete(subscription::table).execute(conn);
    }

    #[actix_rt::test]
    async fn test_insert_restaurant() {
        let conn = init_database().await.get().unwrap();

        let testres: InsertRestaurant = InsertRestaurant {
            smiley_restaurant_id: 1,
            name: String::from("Somewhere"),
            address: String::from("Over"),
            zipcode: String::from("The"),
            city: String::from("Rainbow"),
            cvr: String::from("Is"),
            pnr: String::from("Happiness"),
            latitude: 1.5,
            longitude: 55.2,
        };
        let testid = insert_restaurant(&conn, &testres);
        match restaurant::dsl::restaurant
            .filter(restaurant::smiley_restaurant_id.eq(testres.smiley_restaurant_id))
            .filter(restaurant::name.eq_all(testres.name))
            .filter(restaurant::address.eq_all(testres.address))
            .filter(restaurant::zipcode.eq_all(testres.zipcode))
            .filter(restaurant::city.eq_all(testres.city))
            .filter(restaurant::cvr.eq_all(testres.cvr))
            .filter(restaurant::pnr.eq_all(testres.pnr))
            .filter(restaurant::latitude.eq_all(testres.latitude))
            .filter(restaurant::longitude.eq_all(testres.longitude))
            .first::<Restaurant>(&conn)
        {
            Err(_) => panic!("Error in test for insert of test restaurant"),
            Ok(res) => assert_eq!(res.id, testid),
        }
        clear_database(&conn).await;
    }

    #[actix_rt::test]
    async fn data_loading_test() {
        let conn = init_database().await.get().unwrap();
        load_test_data(&conn).await;
        let addedres: Vec<Restaurant> = restaurant::dsl::restaurant
            .load(&conn)
            .expect("error fetching testdata restaurants in test");
        let addedsmileyreports: Vec<SmileyReport> = smiley_report::dsl::smiley_report
            .load(&conn)
            .expect("error fetching smiley reports in test");
        assert_eq!(addedres.iter().count(), 10);
        assert_eq!(addedsmileyreports.iter().count(), 40);

        clear_database(&conn).await;
    }
}
