use thiserror::Error;

#[derive(Error, Debug)]
pub enum HiveError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("Project error: {0}")]
    Project(String),

    #[error("Tunnel error: {0}")]
    Tunnel(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type HiveResult<T> = Result<T, HiveError>;
