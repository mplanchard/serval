use rusqlite;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApplicationError {}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("Could not load config")]
    Load,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PackageBackendError {
    #[error("Sqlite backend error: {source}")]
    Sqlite {
        #[from]
        source: rusqlite::Error,
    },
}
