use crate::core::filesystem;
use crate::core::project;
use crate::error::{HiveError, HiveResult};
use crate::utils::terminal::*;
use colored::*;

pub fn run(action: EnvAction) -> HiveResult<()> {
    let cwd = project::current_dir();
    let env_path = filesystem::env_path(&cwd);

    if !env_path.exists() {
        return Err(HiveError::Config(
            "No .hive/.env found. Run `hive init` first.".into(),
        ));
    }

    match action {
        EnvAction::List => {
            let content = std::fs::read_to_string(&env_path)?;
            println!();
            print_step("◈", &format!("Environment  —  {}", env_path.display()));
            println!();
            print_divider();
            println!();
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() {
                    println!();
                    continue;
                }
                if line.starts_with('#') {
                    println!("  {}", line.truecolor(80, 80, 80));
                    continue;
                }
                if let Some((key, val)) = line.split_once('=') {
                    let display_val = if val.is_empty() {
                        "(empty)".truecolor(80, 80, 80).to_string()
                    } else {
                        val.truecolor(255, 200, 0).bold().to_string()
                    };
                    println!("  {}  =  {}", key.truecolor(180, 180, 180), display_val);
                }
            }
            println!();
            print_divider();
            println!();
        }

        EnvAction::Get(key) => {
            let content = std::fs::read_to_string(&env_path)?;
            let val = find_key(&content, &key)
                .ok_or_else(|| HiveError::Config(format!("Key '{}' not found in .env", key)))?;
            println!("{}", val);
        }

        EnvAction::Set(key, val) => {
            let content = std::fs::read_to_string(&env_path)?;
            let new_content = set_key(&content, &key, &val);
            std::fs::write(&env_path, new_content)?;
            print_success(&format!(
                "{}  =  {}",
                key.truecolor(180, 180, 180),
                val.truecolor(255, 200, 0).bold()
            ));
        }

        EnvAction::Unset(key) => {
            let content = std::fs::read_to_string(&env_path)?;
            let new_content = remove_key(&content, &key);
            std::fs::write(&env_path, new_content)?;
            print_success(&format!("Removed '{}'", key));
        }
    }

    Ok(())
}

pub enum EnvAction {
    List,
    Get(String),
    Set(String, String),
    Unset(String),
}

fn find_key(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Some(v.trim().trim_matches('"').trim_matches('\'').to_string());
            }
        }
    }
    None
}

fn set_key(content: &str, key: &str, val: &str) -> String {
    let mut found = false;
    let mut lines: Vec<String> = content
        .lines()
        .map(|l| {
            if l.starts_with('#') || l.trim().is_empty() {
                return l.to_string();
            }
            if let Some((k, _)) = l.split_once('=') {
                if k.trim() == key {
                    found = true;
                    return format!("{}={}", key, val);
                }
            }
            l.to_string()
        })
        .collect();

    if !found {
        lines.push(format!("{}={}", key, val));
    }

    lines.join("\n") + "\n"
}

fn remove_key(content: &str, key: &str) -> String {
    content
        .lines()
        .filter(|l| {
            if let Some((k, _)) = l.split_once('=') {
                k.trim() != key
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}
