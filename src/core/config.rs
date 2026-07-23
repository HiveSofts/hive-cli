use crate::error::{HiveError, HiveResult};
use crate::models::config::HiveConfig;
use std::path::Path;

pub fn load(project_root: &Path) -> HiveResult<HiveConfig> {
    let path = crate::core::filesystem::config_path(project_root);
    if !path.exists() {
        return Err(HiveError::Config(
            "No .hive/config.yml found. Run `hive init` first.".into(),
        ));
    }
    let content = crate::core::filesystem::read_file(&path)?;
    let config: HiveConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn save(project_root: &Path, config: &HiveConfig) -> HiveResult<()> {
    let path = crate::core::filesystem::config_path(project_root);
    let content = serde_yaml::to_string(config)?;
    crate::core::filesystem::write_file(&path, &content)?;
    Ok(())
}
