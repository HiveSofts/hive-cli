use crate::error::HiveResult;
use std::path::{Path, PathBuf};

pub fn hive_dir(project_root: &Path) -> PathBuf {
    project_root.join(".hive")
}

pub fn config_path(project_root: &Path) -> PathBuf {
    hive_dir(project_root).join("config.yml")
}

pub fn env_path(project_root: &Path) -> PathBuf {
    hive_dir(project_root).join(".env")
}

pub fn secrets_path(project_root: &Path) -> PathBuf {
    hive_dir(project_root).join(".secrets")
}

pub fn logs_dir(project_root: &Path) -> PathBuf {
    hive_dir(project_root).join("logs")
}

pub fn run_log_path(project_root: &Path) -> PathBuf {
    logs_dir(project_root).join("run.log")
}

pub fn create_hive_dir(project_root: &Path) -> HiveResult<()> {
    let dir = hive_dir(project_root);
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    let logs = logs_dir(project_root);
    if !logs.exists() {
        std::fs::create_dir_all(&logs)?;
    }
    Ok(())
}

pub fn write_file(path: &Path, content: &str) -> HiveResult<()> {
    std::fs::write(path, content)?;
    Ok(())
}

pub fn read_file(path: &Path) -> HiveResult<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn hive_initialized(project_root: &Path) -> bool {
    config_path(project_root).exists()
}
