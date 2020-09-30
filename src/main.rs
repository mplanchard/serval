mod config;
mod error;
mod package;
mod storage;

use std::convert::TryFrom;
use std::error::Error;

use config::Config;
use package::{PackageBackend, PackageBackendConnector, PackageRegistry};
use storage::sqlite;

struct TempRegistry {}
impl PackageRegistry for TempRegistry {
    const NAME: &'static str = "temp";
    const STORAGE_NAMESPACE: &'static str = "temp";

    fn parse_package_data(data: Vec<u8>) -> package::PackageData {
        todo!()
    }

    fn register_api(scope: actix_web::Scope) -> actix_web::Scope {
        todo!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();
    let backend_config = config.backends.get(&config.backend).unwrap().clone();
    let connector = sqlite::SqliteConnector::new(sqlite::SqliteConfig::try_from(backend_config)?);
    let backend = connector.connect()?;
    backend.init::<TempRegistry>()?;

    println!("Hello, world!");
    Ok(())
}
