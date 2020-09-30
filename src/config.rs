use std::collections::HashMap;
use std::iter::FromIterator;

use serde::Deserialize;

pub type BackendConfig = HashMap<String, String>;
pub type BackendConfigs = HashMap<String, BackendConfig>;

#[derive(Deserialize)]
pub struct Config {
    pub backend: String,
    pub backends: BackendConfigs,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            backend: "sqlite".into(),
            backends: default_backend_configs(),
        }
    }
}

fn default_backend_configs() -> BackendConfigs {
    // Define the config as a slice of tuples
    let config = [("sqlite", [("path", "./serval.sqlite")])];
    // Convert it to a hashmap
    HashMap::from_iter(config.iter().map(|i| *i).map(|(k, v)| {
        (
            k.into(),
            HashMap::from_iter(v.iter().map(|i| *i).map(|(k, v)| (k.into(), v.into()))),
        )
    }))
}
