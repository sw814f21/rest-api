#[cfg(test)]
mod tests {
    use crate::database;
    use crate::database::models::*;

    #[actix_rt::test]
    async fn version_ordering() {
        let db_pool = database::new_pool();

        let version_1 = Version::create_new_version(&db_pool.get().unwrap());
        let version_2 = Version::create_new_version(&db_pool.get().unwrap());
        let version_3 = Version::create_new_version(&db_pool.get().unwrap());

        assert_eq!(version_1.id < version_2.id, true);
        assert_eq!(version_2.id < version_3.id, true);
    }

    #[actix_rt::test]
    async fn version_latest_consistency() {
        let db_pool = database::new_pool();

        Version::create_new_version(&db_pool.get().unwrap());
        Version::create_new_version(&db_pool.get().unwrap());
        let version_3 = Version::create_new_version(&db_pool.get().unwrap());

        let current_version = Version::current_version(&db_pool.get().unwrap());

        assert_eq!(version_3.id, current_version.id);
    }

    #[actix_rt::test]
    async fn version_changing_latest() {
        let db_pool = database::new_pool();

        let version_1 = Version::create_new_version(&db_pool.get().unwrap());

        let current_version = Version::current_version(&db_pool.get().unwrap());
        assert_eq!(version_1.id, current_version.id);

        Version::create_new_version(&db_pool.get().unwrap());
        let version_3 = Version::create_new_version(&db_pool.get().unwrap());

        let current_version = Version::current_version(&db_pool.get().unwrap());
        assert_eq!(version_3.id, current_version.id);
    }
}
