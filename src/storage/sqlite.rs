//! SQLite storage backend

use rusqlite::Connection;
use serde::Deserialize;
use std::convert::TryFrom;

use crate::config::BackendConfig;
use crate::error::ConfigError;
use crate::error::PackageBackendError;
use crate::package::{
    PackageBackend, PackageBackendConnector, PackageData, PackageInfo, PackageQuery,
    PackageRegistry,
};

#[derive(Deserialize, Debug)]
pub struct SqliteConfig {
    path: String,
}
impl TryFrom<BackendConfig> for SqliteConfig {
    type Error = ConfigError;

    fn try_from(value: BackendConfig) -> Result<Self, Self::Error> {
        Ok(SqliteConfig {
            path: value.get("path").ok_or(ConfigError::Load)?.into(),
        })
    }
}

pub struct SqliteConnector {
    path: String,
}
impl PackageBackendConnector for SqliteConnector {
    const NAME: &'static str = "sqlite";
    type Backend = SqliteBackend;
    type Config = SqliteConfig;

    fn connect(self) -> Result<Self::Backend, PackageBackendError> {
        Ok(SqliteBackend::new(Connection::open(&self.path)?))
    }

    fn new(config: Self::Config) -> Self {
        Self { path: config.path }
    }
}

pub struct SqliteBackend {
    connection: Connection,
}
impl SqliteBackend {
    fn new(connection: Connection) -> Self {
        Self { connection }
    }
}
impl PackageBackend for SqliteBackend {
    fn init<R: PackageRegistry>(self) -> Result<(), PackageBackendError> {
        self.connection.execute_batch(&format!(
            r#"CREATE TABLE IF NOT EXISTS {name} (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                created TEXT CURRENT_TIMESTAMP
            );
            CREATE INDEX IF NOT EXISTS {name}_nm ON {name}(name);
            CREATE INDEX IF NOT EXISTS {name}_ver ON {name}(version);
            CREATE UNIQUE INDEX IF NOT EXISTS
                {name}_nm_ver_uq ON {name}(name, version);

            CREATE TABLE IF NOT EXISTS {name}_meta (
                package INTEGER NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                FOREIGN KEY(package) REFERENCES {name}(id)
            );
            "#,
            name = <R as PackageRegistry>::NAME
        ))?;
        Ok(())
    }

    fn save<R: PackageRegistry>(
        self,
        registry: R,
        data: PackageData,
    ) -> Result<PackageInfo, PackageBackendError> {
        todo!()
    }

    fn all<R: PackageRegistry>(self, registry: R) -> Result<Vec<PackageInfo>, PackageBackendError> {
        todo!()
    }

    fn find<R: PackageRegistry>(
        self,
        registry: R,
        query: PackageQuery,
    ) -> Result<Vec<PackageInfo>, PackageBackendError> {
        todo!()
    }

    fn get<R: PackageRegistry>(
        self,
        registry: R,
        info: PackageInfo,
    ) -> Result<PackageData, PackageBackendError> {
        todo!()
    }

    fn remove<R: PackageRegistry>(
        self,
        registry: R,
        info: PackageInfo,
    ) -> Result<PackageInfo, PackageBackendError> {
        todo!()
    }

    fn replace<R: PackageRegistry>(
        self,
        registry: R,
        data: PackageData,
    ) -> Result<PackageInfo, PackageBackendError> {
        todo!()
    }
}
