use crate::error::{ConfigError, PackageBackendError};
use actix_web::Scope;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;

use crate::config::BackendConfig;

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

pub trait PackageRegistry {
    const NAME: &'static str;
    const STORAGE_NAMESPACE: &'static str;

    fn parse_package_data(data: Vec<u8>) -> PackageData;

    fn register_api(scope: Scope) -> Scope;
}

pub trait PackageBackendConnector {
    const NAME: &'static str;
    type Backend: PackageBackend;
    type Config: TryFrom<BackendConfig, Error = ConfigError>;

    fn new(config: Self::Config) -> Self;

    fn connect(self) -> Result<Self::Backend, PackageBackendError>;
}

pub trait PackageBackend: Sized {
    fn init<R: PackageRegistry>(self) -> Result<(), PackageBackendError>;
    fn save<R: PackageRegistry>(
        self,
        registry: R,
        data: PackageData,
    ) -> Result<PackageInfo, PackageBackendError>;
    fn all<R: PackageRegistry>(self, registry: R) -> Result<Vec<PackageInfo>, PackageBackendError>;
    fn find<R: PackageRegistry>(
        self,
        registry: R,
        query: PackageQuery,
    ) -> Result<Vec<PackageInfo>, PackageBackendError>;
    fn get<R: PackageRegistry>(
        self,
        registry: R,
        info: PackageInfo,
    ) -> Result<PackageData, PackageBackendError>;
    fn remove<R: PackageRegistry>(
        self,
        registry: R,
        info: PackageInfo,
    ) -> Result<PackageInfo, PackageBackendError>;
    fn replace<R: PackageRegistry>(
        self,
        registry: R,
        data: PackageData,
    ) -> Result<PackageInfo, PackageBackendError>;
}
