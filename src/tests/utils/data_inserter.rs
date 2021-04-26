#[cfg(test)]
mod tests {
    use crate::database;
    use crate::database::models::*;
    use crate::utils::data_inserter;
    use crate::utils::json_parser;

    #[actix_rt::test]
    async fn restaurant_removal() {
        let db_pool = database::new_pool();

        let restaurant = test_restaurant();

        let version = Version::get_from_token(&db_pool.get().unwrap(), "1");
        let id = data_inserter::insert_restaurant(&db_pool.get().unwrap(), &restaurant, version.id);

        let version2 = Version::get_from_token(&db_pool.get().unwrap(), "2");
        data_inserter::remove_restaurant(&db_pool.get().unwrap(), id, &version2);

        let res_vec = Restaurant::get_all_resturants(&db_pool.get().unwrap());
        let removals = RemovedRestaurant::get_removals_since(&db_pool.get().unwrap(), version.id);

        assert_eq!(res_vec.len(), 0);
        assert_eq!(removals.len(), 1);
    }

    #[actix_rt::test]
    async fn restaurant_versioning() {
        let db_pool = database::new_pool();

        let restaurant = test_restaurant();

        let version_1 = Version::get_from_token(&db_pool.get().unwrap(), "1");
        data_inserter::insert_restaurant(&db_pool.get().unwrap(), &restaurant, version_1.id);

        let version_2 = Version::get_from_token(&db_pool.get().unwrap(), "2");
        data_inserter::insert_restaurant(&db_pool.get().unwrap(), &restaurant, version_2.id);

        let res_total = Restaurant::get_all_resturants(&db_pool.get().unwrap());
        let res_v1 = Restaurant::get_since_version(&db_pool.get().unwrap(), version_1.id);
        let res_v2 = Restaurant::get_since_version(&db_pool.get().unwrap(), version_2.id);

        assert_eq!(res_total.len(), 2);
        assert_eq!(res_v1.len(), 1);
        assert_eq!(res_v2.len(), 0);
    }

    #[actix_rt::test]
    async fn restaurant_updating() {
        let db_pool = database::new_pool();

        let mut restaurant = test_restaurant();

        let version_1 = Version::get_from_token(&db_pool.get().unwrap(), "1");
        data_inserter::insert_restaurant(&db_pool.get().unwrap(), &restaurant, version_1.id);

        restaurant.name = String::from("some other name");
        let version_2 = Version::get_from_token(&db_pool.get().unwrap(), "2");
        data_inserter::update_restaurant(&db_pool.get().unwrap(), &restaurant, version_2.id);

        let changed_restaurant =
            Restaurant::get_since_version(&db_pool.get().unwrap(), version_1.id).remove(0);

        assert_eq!(changed_restaurant.name, "some other name");
    }

    #[actix_rt::test]
    async fn restaurant_updating_versioning() {
        let db_pool = database::new_pool();

        let mut restaurant = test_restaurant();

        let version_1 = Version::get_from_token(&db_pool.get().unwrap(), "1");
        data_inserter::insert_restaurant(&db_pool.get().unwrap(), &restaurant, version_1.id);

        restaurant.smiley_restaurant_id = String::from("5435345"); // Change id for other restaurant
        data_inserter::insert_restaurant(&db_pool.get().unwrap(), &restaurant, version_1.id);

        restaurant.name = String::from("some other name");
        let version_2 = Version::get_from_token(&db_pool.get().unwrap(), "2");
        data_inserter::update_restaurant(&db_pool.get().unwrap(), &restaurant, version_2.id);

        let res_v1 = Restaurant::get_since_version(&db_pool.get().unwrap(), version_1.id);
        let res_total = Restaurant::get_all_resturants(&db_pool.get().unwrap());

        assert_eq!(res_v1.len(), 1);
        assert_eq!(res_total.len(), 2);
    }

    fn test_restaurant() -> json_parser::JsonRestaurant {
        json_parser::JsonRestaurant {
            city: String::from("test"),
            cvr: String::from("15454331"),
            latitude: 32.0,
            longitude: 13.0,
            pnr: String::from("64848234"),
            address: String::from("someting vej 3"),
            zipcode: String::from("3145"),
            name: String::from("Fishing fish grill"),
            smiley_restaurant_id: String::from("42545"),
            smiley_reports: Vec::new(),
        }
    }
}
