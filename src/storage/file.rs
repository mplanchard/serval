use std::ffi::OsString;

use crate::traits::{PackageData, PackageInfo, PackageQuery, StorageBackend};

struct FileBackend {
    root_dir: OsString,
}
impl StorageBackend for FileBackend {
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
