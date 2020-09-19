//! SQLite storage backend

use crate::traits::{
    PackageData, PackageInfo, PackageQuery, StorageBackend, StorageBackendConnector,
};

struct SqliteConnector {}
impl StorageBackendConnector for SqliteConnector {
    type Backend = SqliteBackend;

    fn connect() -> Self::Backend {
        todo!()
    }
}

struct SqliteBackend {}
impl SqliteBackend {}
impl StorageBackend for SqliteBackend {
    fn save_package(data: PackageData) -> Result<PackageInfo, String> {
        todo!()
    }

    fn all_packages() -> Result<Vec<PackageInfo>, ()> {
        todo!()
    }

    fn find_packages(query: PackageQuery) -> Result<Vec<PackageInfo>, ()> {
        todo!()
    }

    fn get_package(info: PackageInfo) -> Result<PackageData, ()> {
        todo!()
    }

    fn remove_package(info: PackageInfo) -> Result<PackageInfo, ()> {
        todo!()
    }

    fn replace_package(data: PackageData) -> Result<PackageInfo, ()> {
        todo!()
    }
}
