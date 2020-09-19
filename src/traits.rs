use actix_web::Scope;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct PackageData {
    data: Vec<u8>,
    info: PackageInfo,
}

pub struct PackageInfo {
    name: String,
    version: String,
    meta: Option<HashMap<String, String>>,
}

pub struct PackageQuery {
    name: Option<String>,
    version: Option<String>,
    meta: Option<HashMap<String, String>>,
}

pub trait PackageRegistry<S>
where
    S: StorageBackend,
{
    const NAME: &'static str;

    fn parse_package_data(data: Vec<u8>) -> PackageData;

    fn register_api(scope: Scope) -> Scope;

    fn init_storage(backend: &S) -> Result<(), String>;
}

pub trait StorageBackendConnector {
    type Backend: StorageBackend;

    fn connect() -> Self::Backend;
}

pub trait StorageBackend {
    fn save_package(data: PackageData) -> Result<PackageInfo, String>;
    fn all_packages() -> Result<Vec<PackageInfo>, ()>;
    fn find_packages(query: PackageQuery) -> Result<Vec<PackageInfo>, ()>;
    fn get_package(info: PackageInfo) -> Result<PackageData, ()>;
    fn remove_package(info: PackageInfo) -> Result<PackageInfo, ()>;
    fn replace_package(data: PackageData) -> Result<PackageInfo, ()>;
}

struct AppData<R, S>
where
    R: PackageRegistry<S>,
    S: StorageBackend,
{
    registry: R,
    _marker: PhantomData<S>,
}
