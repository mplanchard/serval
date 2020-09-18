use actix_web::Scope;
use std::collections::HashMap;
use std::marker::PhantomData;

struct PackageData {
    data: Vec<u8>,
    name: String,
    version: String,
    meta: Option<HashMap<String, String>>,
}

trait PackageRegistry<S>
where
    S: StorageBackend,
{
    const NAME: &'static str;

    fn save_package(backend: S, data: PackageData) -> Result<(), String>;
}

trait StorageBackend {}

struct AppData<R, S>
where
    R: PackageRegistry<S>,
    S: StorageBackend,
{
    registry: R,
    _marker: PhantomData<S>,
}

trait RegistryApi {
    fn register(scope: Scope) -> Scope;
}
