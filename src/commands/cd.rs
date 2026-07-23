use crate::core::registry;
use crate::error::{HiveError, HiveResult};

pub fn run(query: &str) -> HiveResult<()> {
    if let Some(entry) = registry::find_project(query)? {
        println!("cd \"{}\"", entry.path);
        return Ok(());
    }

    if registry::global_hive_exists() {
        if let Ok(projects) = registry::get_hive_app_projects() {
            for p in &projects {
                let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let path = p.get("path").and_then(|v| v.as_str()).unwrap_or("");
                if name.to_lowercase().contains(&query.to_lowercase()) {
                    println!("cd \"{}\"", path);
                    return Ok(());
                }
            }
        }
    }

    Err(HiveError::Project(format!(
        "No project matching '{}'",
        query
    )))
}
