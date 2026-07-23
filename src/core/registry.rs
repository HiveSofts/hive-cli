use crate::error::{HiveError, HiveResult};
use crate::models::config::HiveConfig;
use crate::utils::terminal::{print_step, print_warn};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryEntry {
    pub name: String,
    pub description: Option<String>,
    pub framework: Option<String>,
    pub path: String,
    pub port: Option<u16>,
    pub created_at: String,
    pub author: Option<String>,
}

pub fn hive_home() -> PathBuf {
    dirs_home().join(".hive")
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn global_hive_exists() -> bool {
    hive_home().exists()
}

pub fn cli_projects_dir() -> PathBuf {
    hive_home().join(".cli").join("projects")
}

pub fn ensure_cli_projects_dir() -> HiveResult<()> {
    let dir = cli_projects_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(())
}

pub fn warn_if_no_global_hive() {
    if !global_hive_exists() {
        print_warn("Hive app not found at ~/.hive");
        print_step(
            "→",
            "Install Hive desktop app for the full experience: https://hive.dev",
        );
        println!();
    }
}

pub fn register_project(
    project_root: &Path,
    config: &HiveConfig,
    port: Option<u16>,
) -> HiveResult<()> {
    ensure_cli_projects_dir()?;

    let entry = RegistryEntry {
        name: config.project.name.clone(),
        description: config.project.description.clone(),
        framework: config.project.framework.clone(),
        path: project_root.to_string_lossy().to_string(),
        port,
        created_at: Local::now().to_rfc3339(),
        author: config.project.author.clone(),
    };

    let filename = sanitize_name(&config.project.name);
    let path = cli_projects_dir().join(format!("{}.yml", filename));
    let content = serde_yaml::to_string(&entry)?;
    std::fs::write(&path, content)?;
    Ok(())
}

pub fn list_projects() -> HiveResult<Vec<(String, RegistryEntry)>> {
    let dir = cli_projects_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut projects = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("yml") {
            let content = std::fs::read_to_string(&path)?;
            if let Ok(reg) = serde_yaml::from_str::<RegistryEntry>(&content) {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                projects.push((name, reg));
            }
        }
    }
    Ok(projects)
}

pub fn find_project(query: &str) -> HiveResult<Option<RegistryEntry>> {
    let projects = list_projects()?;
    let q = query.to_lowercase();
    for (_, entry) in projects {
        if entry.name.to_lowercase().contains(&q) || entry.path.to_lowercase().contains(&q) {
            return Ok(Some(entry));
        }
    }
    Ok(None)
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .to_lowercase()
}

pub fn get_hive_app_projects() -> HiveResult<Vec<serde_json::Value>> {
    let projects_dir = hive_home().join("projects");
    if !projects_dir.exists() {
        return Ok(vec![]);
    }
    let mut projects = vec![];
    for entry in std::fs::read_dir(&projects_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let content = std::fs::read_to_string(&path)?;
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                projects.push(val);
            }
        }
    }
    Ok(projects)
}
