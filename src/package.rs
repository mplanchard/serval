use crate::error::{ConfigError, PackageBackendError};
use actix_web::Scope;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;

use crate::config::BackendConfig;

pub struct PackageData {
    pub data: Vec<u8>,
    pub info: PackageInfo,
}
impl PackageData {
    pub fn new(data: Vec<u8>, info: PackageInfo) -> Self {
        PackageData { data, info }
    }
}

pub type PackageMeta = HashMap<String, String>;

pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub meta: Option<PackageMeta>,
}
impl PackageInfo {
    pub fn new(name: String, version: String, meta: Option<PackageMeta>) -> Self {
        PackageInfo {
            name,
            version,
            meta,
        }
    }
}

pub struct PackageQuery {
    name: Option<String>,
    version: Option<String>,
    meta: Option<HashMap<String, String>>,
}

pub trait PackageRegistry {
    const NAME: &'static str;
    const STORAGE_NAMESPACE: &'static str;

    fn parse_package_data(data: Vec<u8>) -> PackageData;

    fn register_api(scope: Scope) -> Scope;
}

pub trait PackageBackendConnector {
    const NAME: &'static str;
    type Config: TryFrom<BackendConfig, Error = ConfigError>;
    type Initializer: PackageBackendInitializer;

    fn new(config: Self::Config) -> Self;

    fn connect(self) -> Result<Self::Initializer, PackageBackendError>;
}

pub trait PackageBackendInitializer {
    fn init<R: PackageRegistry + 'static>(
        self,
        registry: R,
    ) -> Result<Box<dyn PackageBackend<Registry = R>>, PackageBackendError>;
}

pub trait PackageBackend {
    type Registry: PackageRegistry;
    fn save(&mut self, data: PackageData) -> Result<PackageInfo, PackageBackendError>;
    fn all(&self) -> Result<Vec<PackageInfo>, PackageBackendError>;
    fn count(&self) -> Result<i64, PackageBackendError>;
    fn find(
        &self,
        query: PackageQuery,
    ) -> Result<Vec<PackageInfo>, PackageBackendError>;
    fn get(&self, info: PackageInfo) -> Result<PackageData, PackageBackendError>;
    fn remove(&self, info: PackageInfo) -> Result<PackageInfo, PackageBackendError>;
    fn replace(&self, data: PackageData) -> Result<PackageInfo, PackageBackendError>;
}
