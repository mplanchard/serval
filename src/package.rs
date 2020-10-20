use async_trait::async_trait;
use std::collections::HashMap;

use crate::config::BackendConfig;
use crate::error::PackageBackendError;

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

pub trait PackageRegistry: Send {
    const NAME: &'static str;
    const STORAGE_NAMESPACE: &'static str;

    fn parse_package_data(data: Vec<u8>) -> PackageData;
}

pub trait PackageBackend {
    const NAME: &'static str;
    type Executor: PackageBackendExecutor;

    fn config(config: BackendConfig) -> Self;

    fn connect(self) -> Result<Self::Executor, PackageBackendError>;
}

#[async_trait]
pub trait PackageBackendExecutor {
    async fn init(
        &self,
        registry: &impl PackageRegistry,
    ) -> Result<(), PackageBackendError>;
    async fn save(
        &self,
        registry: &impl PackageRegistry,
        data: PackageData,
    ) -> Result<PackageInfo, PackageBackendError>;
    async fn all(
        &self,
        registry: &impl PackageRegistry,
    ) -> Result<Vec<PackageInfo>, PackageBackendError>;
    async fn count(
        &self,
        registry: &impl PackageRegistry,
    ) -> Result<i64, PackageBackendError>;
    async fn find(
        &self,
        registry: &impl PackageRegistry,
        query: &PackageQuery,
    ) -> Result<Vec<PackageInfo>, PackageBackendError>;
    async fn get(
        &self,
        registry: &impl PackageRegistry,
        info: &PackageInfo,
    ) -> Result<PackageData, PackageBackendError>;
    async fn remove(
        &self,
        registry: &impl PackageRegistry,
        info: &PackageInfo,
    ) -> Result<PackageInfo, PackageBackendError>;
    async fn replace(
        &self,
        registry: &impl PackageRegistry,
        data: &PackageData,
    ) -> Result<PackageInfo, PackageBackendError>;
}
