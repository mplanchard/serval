use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct BackendConfig {
    pub url: String,
}
impl BackendConfig {
    pub fn new<S: Into<String>>(url: S) -> Self {
        Self { url: url.into() }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub backend: String,
    pub backends: HashMap<String, BackendConfig>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            backend: "sqlite".into(),
            backends: HashMap::from,
        }
    }
}
impl Config {
    fn default_backend_configs() -> HashMap<String, BackendConfig> {
        let mut configs = HashMap::new();
        configs.insert("sqlite".into(), BackendConfig::new("./serval.sqlite"));
        configs
    }
}
