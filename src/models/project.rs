use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Framework {
    Laravel,
    NextJs,
    React,
    Vue,
    Nuxt,
    Node,
    Python,
    Django,
    FastAPI,
    Rust,
    Go,
    Unknown,
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Framework::Laravel => write!(f, "Laravel"),
            Framework::NextJs => write!(f, "Next.js"),
            Framework::React => write!(f, "React"),
            Framework::Vue => write!(f, "Vue"),
            Framework::Nuxt => write!(f, "Nuxt"),
            Framework::Node => write!(f, "Node.js"),
            Framework::Python => write!(f, "Python"),
            Framework::Django => write!(f, "Django"),
            Framework::FastAPI => write!(f, "FastAPI"),
            Framework::Rust => write!(f, "Rust"),
            Framework::Go => write!(f, "Go"),
            Framework::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectMeta {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub has_docker: bool,
    pub has_env: bool,
    pub extra_services: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub framework: Framework,
    pub path: std::path::PathBuf,
    pub meta: ProjectMeta,
}
