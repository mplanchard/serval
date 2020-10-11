//! SQLite storage backend

use rusqlite::{params, Connection, Error as SqliteError, NO_PARAMS};
use serde::Deserialize;
use std::convert::TryFrom;

use crate::config::BackendConfig;
use crate::error::ConfigError;
use crate::error::PackageBackendError;
use crate::package::{
    PackageBackend, PackageBackendConnector, PackageBackendInitializer, PackageData,
    PackageInfo, PackageQuery, PackageRegistry,
};

#[derive(Clone, Deserialize, Debug)]
pub struct SqliteConfig {
    path: String,
}
impl SqliteConfig {
    fn new<S: Into<String>>(path: S) -> Self {
        SqliteConfig { path: path.into() }
    }
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
    type Config = SqliteConfig;
    type Initializer = SqliteInitializer;

    fn connect(self) -> Result<SqliteInitializer, PackageBackendError> {
        Ok(SqliteInitializer::new(Connection::open(&self.path)?))
    }

    fn new(config: Self::Config) -> Self {
        Self { path: config.path }
    }
}

pub struct SqliteInitializer {
    connection: Connection,
}
impl SqliteInitializer {
    fn new(connection: Connection) -> Self {
        Self { connection }
    }
}
impl PackageBackendInitializer for SqliteInitializer {
    fn init<R: PackageRegistry + 'static>(
        self,
        registry: R,
    ) -> Result<Box<dyn PackageBackend<Registry = R>>, PackageBackendError> {
        self.connection.execute_batch(&format!(
            r#"CREATE TABLE IF NOT EXISTS {name} (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                data BLOB NOT NULL,
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
            name = <R as PackageRegistry>::STORAGE_NAMESPACE
        ))?;
        Ok(Box::new(SqliteBackend::new(self.connection, registry)))
    }
}

pub struct SqliteBackend<R: PackageRegistry> {
    connection: Connection,
    registry: R,
}
impl<R: PackageRegistry> SqliteBackend<R> {
    fn new(connection: Connection, registry: R) -> Self {
        Self {
            connection,
            registry,
        }
    }
}
impl<R: PackageRegistry> PackageBackend for SqliteBackend<R> {
    type Registry = R;

    fn save(&mut self, data: PackageData) -> Result<PackageInfo, PackageBackendError> {
        dbg!("here!");
        println!("here!");
        let tx = self.connection.transaction()?;
        tx.execute(
            &format!(
                r#"INSERT INTO {table} (name, version, data)
                VALUES (?1, ?2, ?3)"#,
                table = Self::Registry::STORAGE_NAMESPACE,
            ),
            params![&data.info.name, &data.info.version, data.data],
        )?;
        let row_id = tx.last_insert_rowid();
        let meta_table = format!("{}_meta", Self::Registry::STORAGE_NAMESPACE);
        if let Some(meta) = &data.info.meta {
            meta.iter()
                .map(|(k, v)| {
                    dbg!("now here!");
                    tx.execute(
                        &format!(
                            r#"INSERT INTO {table} (package, key, value)
                            VALUES (?1, ?2, ?3)"#,
                            table = meta_table,
                        ),
                        params![k, v, row_id],
                    )
                })
                .collect::<Result<Vec<_>, SqliteError>>()?;
        }
        tx.commit()?;
        Ok(data.info)
    }

    fn all(&self) -> Result<Vec<PackageInfo>, PackageBackendError> {
        todo!()
    }

    fn count(&self) -> Result<i64, PackageBackendError> {
        Ok(self.connection.query_row(
            &format!(
                "SELECT count(*) from {table}",
                table = Self::Registry::STORAGE_NAMESPACE
            ),
            NO_PARAMS,
            |r| r.get(0),
        )?)
    }

    fn find(
        &self,
        query: PackageQuery,
    ) -> Result<Vec<PackageInfo>, PackageBackendError> {
        todo!()
    }

    fn get(&self, info: PackageInfo) -> Result<PackageData, PackageBackendError> {
        todo!()
    }

    fn remove(&self, info: PackageInfo) -> Result<PackageInfo, PackageBackendError> {
        todo!()
    }

    fn replace(&self, data: PackageData) -> Result<PackageInfo, PackageBackendError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn pkg_data() -> PackageData {
        let mut meta: HashMap<String, String> = HashMap::new();
        meta.insert("foo".into(), "foo".into());
        meta.insert("bar".into(), "bar".into());
        PackageData::new(
            vec![0, 1, 2, 3, 4],
            PackageInfo::new("foopkg".into(), "1.0.0".into(), Some(meta)),
        )
    }

    fn get_initializer(tmpdir: &tempfile::TempDir) -> (SqliteInitializer, Connection) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let path = format!(
            "{}/{}",
            tmpdir.path().to_str().unwrap(),
            format!("tmp-{}.sqlite", now)
        );
        let conf = SqliteConfig::new(&path);
        let be = SqliteConnector::new(conf).connect().unwrap();
        let conn = Connection::open(&path).unwrap();
        (be, conn)
    }

    fn get_backend(
        tmpdir: &tempfile::TempDir,
    ) -> (Box<dyn PackageBackend<Registry = TestRegistry>>, Connection) {
        let (backend, connection) = get_initializer(tmpdir);
        let be = backend.init(TestRegistry {}).unwrap();
        (be, connection)
    }

    struct TestRegistry {}
    impl PackageRegistry for TestRegistry {
        const NAME: &'static str = "temp";
        const STORAGE_NAMESPACE: &'static str = "temp";

        fn parse_package_data(data: Vec<u8>) -> PackageData {
            todo!()
        }

        fn register_api(scope: actix_web::Scope) -> actix_web::Scope {
            todo!()
        }
    }

    #[test]
    fn init_backend() {
        let tmpdir = tempfile::tempdir().unwrap();
        let (backend, connection) = get_initializer(&tmpdir);
        // backend.init().unwrap();
        backend.init(TestRegistry {}).unwrap();

        // We get no rows back from the inited tables.
        assert_eq!(
            connection
                .query_row::<i32, _, _>(
                    &format!(
                        "SELECT count(*) FROM {}",
                        TestRegistry::STORAGE_NAMESPACE
                    ),
                    NO_PARAMS,
                    |r| r.get(0),
                )
                .unwrap(),
            0
        );
        assert_eq!(
            connection
                .query_row::<i32, _, _>(
                    &format!(
                        "SELECT count(*) FROM {}_meta",
                        TestRegistry::STORAGE_NAMESPACE
                    ),
                    NO_PARAMS,
                    |r| r.get(0),
                )
                .unwrap(),
            0
        );
    }

    #[test]
    fn save() {
        let tmpdir = tempfile::tempdir().unwrap();
        let (mut backend, connection) = get_backend(&tmpdir);
        let pkg = pkg_data();

        assert_eq!(backend.count().unwrap(), 0);
        let info = backend.save(pkg).unwrap();
        assert_eq!(backend.count().unwrap(), 1);
    }
}
