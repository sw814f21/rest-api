#[cfg(test)]
mod tests {
    use crate::database;
    use crate::database::models::*;

    #[actix_rt::test]
    async fn version_ordering() {
        let db_pool = database::new_pool();

        let version_1 = Version::get_from_token(&db_pool.get().unwrap(), "1");
        let version_2 = Version::get_from_token(&db_pool.get().unwrap(), "2");
        let version_3 = Version::get_from_token(&db_pool.get().unwrap(), "3");

        assert_eq!(version_1.id < version_2.id, true);
        assert_eq!(version_2.id < version_3.id, true);
    }

    #[actix_rt::test]
    async fn version_latest_consistency() {
        let db_pool = database::new_pool();

        Version::get_from_token(&db_pool.get().unwrap(), "1");
        Version::get_from_token(&db_pool.get().unwrap(), "2");
        let version_3 = Version::get_from_token(&db_pool.get().unwrap(), "3");

        let current_version = Version::current_version(&db_pool.get().unwrap());

        assert_eq!(version_3.id, current_version.id);
    }

    #[actix_rt::test]
    async fn version_changing_latest() {
        let db_pool = database::new_pool();

        let version_1 = Version::get_from_token(&db_pool.get().unwrap(), "1");

        let current_version = Version::current_version(&db_pool.get().unwrap());
        assert_eq!(version_1.id, current_version.id);

        Version::get_from_token(&db_pool.get().unwrap(), "2");
        let version_3 = Version::get_from_token(&db_pool.get().unwrap(), "3");

        let current_version = Version::current_version(&db_pool.get().unwrap());
        assert_eq!(version_3.id, current_version.id);
    }
}
