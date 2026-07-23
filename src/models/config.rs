use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceConfig {
    pub port: Option<u16>,
    pub command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub framework: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RunScript {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HiveConfig {
    pub project: ProjectConfig,

    #[serde(default)]
    pub services: IndexMap<String, ServiceConfig>,

    #[serde(default)]
    pub scripts: IndexMap<String, RunScript>,
}
