use crate::core::{config, filesystem, project};
use crate::error::{HiveError, HiveResult};
use crate::utils::terminal::*;
use colored::*;

pub enum ConfigAction {
    Show,
    Set(String, String),
}

pub fn run(action: ConfigAction) -> HiveResult<()> {
    let cwd = project::current_dir();

    if !filesystem::hive_initialized(&cwd) {
        return Err(HiveError::Config(
            "No .hive/config.yml found. Run `hive init` first.".into(),
        ));
    }

    match action {
        ConfigAction::Show => {
            let path = filesystem::config_path(&cwd);
            let raw = std::fs::read_to_string(&path)?;
            println!();
            print_step("◈", &format!("Config  —  {}", path.display()));
            println!();
            print_divider();
            println!();
            for line in raw.lines() {
                if line.trim_start().starts_with('#') {
                    println!("  {}", line.truecolor(70, 70, 70));
                } else if line.contains(':') && !line.trim_start().starts_with('-') {
                    let mut parts = line.splitn(2, ':');
                    let key = parts.next().unwrap_or("");
                    let val = parts.next().unwrap_or("");
                    print!("  {}", key.truecolor(147, 197, 253));
                    println!(":{}", val.truecolor(220, 220, 220));
                } else {
                    println!("  {}", line.truecolor(180, 180, 180));
                }
            }
            println!();
            print_divider();
            println!();
        }

        ConfigAction::Set(key, value) => {
            let mut cfg = config::load(&cwd)?;

            match key.as_str() {
                "project.name" => cfg.project.name = value.clone(),
                "project.description" => cfg.project.description = Some(value.clone()),
                "project.author" => cfg.project.author = Some(value.clone()),
                "project.framework" => cfg.project.framework = Some(value.clone()),
                _ => {
                    return Err(HiveError::Config(format!(
                        "Unknown config key '{}'. Supported: project.name, project.description, project.author, project.framework",
                        key
                    )));
                }
            }

            config::save(&cwd, &cfg)?;
            print_success(&format!(
                "{}  =  {}",
                key.truecolor(180, 180, 180).to_string().as_str(),
                value.truecolor(255, 200, 0).bold().to_string().as_str()
            ));
        }
    }

    Ok(())
}
