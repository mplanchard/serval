//! SQLite storage backend

use async_trait::async_trait;
use serde::Deserialize;
use sqlx;
use std::convert::TryFrom;

use crate::config::BackendConfig;
use crate::error::ConfigError;
use crate::error::PackageBackendError;
use crate::package::{
    PackageBackend, PackageBackendExecutor, PackageData, PackageInfo, PackageQuery,
    PackageRegistry,
};

pub struct PackageBackendSqlite {
    path: String,
}
impl PackageBackend for PackageBackendSqlite {
    const NAME: &'static str = "sqlite";
    type Executor = PackageBackendSqliteExecutor;

    fn connect(self) -> Result<PackageBackendSqliteExecutor, PackageBackendError> {
        todo!()
    }

    fn config(config: BackendConfig) -> Self {
        Self { path: config.url }
    }
}

pub struct PackageBackendSqliteExecutor {
    connection: sqlx::SqlitePool,
}
impl PackageBackendSqliteExecutor {
    fn new(connection: sqlx::SqlitePool) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl PackageBackendExecutor for PackageBackendSqliteExecutor {
    async fn init(
        &self,
        registry: &impl PackageRegistry,
    ) -> Result<(), PackageBackendError> {
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
    async fn save(
        &self,
        registry: &impl PackageRegistry,
        data: PackageData,
    ) -> Result<PackageInfo, PackageBackendError> {
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

    async fn all(
        &self,
        registry: &impl PackageRegistry,
    ) -> Result<Vec<PackageInfo>, PackageBackendError> {
        todo!()
    }

    async fn count(
        &self,
        registry: &impl PackageRegistry,
    ) -> Result<i64, PackageBackendError> {
        Ok(self.connection.query_row(
            &format!(
                "SELECT count(*) from {table}",
                table = Self::Registry::STORAGE_NAMESPACE
            ),
            NO_PARAMS,
            |r| r.get(0),
        )?)
    }

    async fn find(
        &self,
        registry: &impl PackageRegistry,
        query: PackageQuery,
    ) -> Result<Vec<PackageInfo>, PackageBackendError> {
        todo!()
    }

    async fn get(
        &self,
        registry: &impl PackageRegistry,
        info: PackageInfo,
    ) -> Result<PackageData, PackageBackendError> {
        todo!()
    }

    async fn remove(
        &self,
        registry: &impl PackageRegistry,
        info: PackageInfo,
    ) -> Result<PackageInfo, PackageBackendError> {
        todo!()
    }

    async fn replace(
        &self,
        registry: &impl PackageRegistry,
        data: PackageData,
    ) -> Result<PackageInfo, PackageBackendError> {
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
