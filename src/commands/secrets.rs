use crate::core::filesystem;
use crate::core::project;
use crate::error::{HiveError, HiveResult};
use crate::utils::terminal::*;
use colored::*;

pub fn run(action: SecretsAction) -> HiveResult<()> {
    let cwd = project::current_dir();
    let secrets_path = filesystem::secrets_path(&cwd);

    if !secrets_path.exists() {
        return Err(HiveError::Config(
            "No .hive/.secrets found. Run `hive init` first.".into(),
        ));
    }

    match action {
        SecretsAction::List => {
            let content = std::fs::read_to_string(&secrets_path)?;
            println!();
            print_step("🔐", "Secrets");
            println!();
            print_warn("Values are masked. Use `hive secrets get KEY` to reveal.");
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
                    let masked = if val.is_empty() {
                        "(empty)".truecolor(80, 80, 80).to_string()
                    } else {
                        "●●●●●●●●".truecolor(100, 100, 100).to_string()
                    };
                    println!("  {}  =  {}", key.truecolor(180, 180, 180), masked);
                }
            }
            println!();
            print_divider();
            println!();
        }

        SecretsAction::Get(key) => {
            let content = std::fs::read_to_string(&secrets_path)?;
            let val = find_key(&content, &key)
                .ok_or_else(|| HiveError::Config(format!("Secret '{}' not found", key)))?;
            if val.is_empty() {
                print_warn(&format!("'{}' is set but empty", key));
            } else {
                println!("{}", val);
            }
        }

        SecretsAction::Set(key, val) => {
            let content = std::fs::read_to_string(&secrets_path)?;
            let new_content = set_key(&content, &key, &val);
            std::fs::write(&secrets_path, new_content)?;
            print_success(&format!(
                "Secret '{}' saved  {}",
                key,
                "●●●●●●●●".truecolor(100, 100, 100)
            ));
        }

        SecretsAction::Unset(key) => {
            let content = std::fs::read_to_string(&secrets_path)?;
            let new_content = remove_key(&content, &key);
            std::fs::write(&secrets_path, new_content)?;
            print_success(&format!("Secret '{}' removed", key));
        }

        SecretsAction::Export => {
            let content = std::fs::read_to_string(&secrets_path)?;
            println!();
            print_step("📤", "Exporting secrets as shell exports");
            println!();
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with('#') || line.is_empty() {
                    continue;
                }
                if let Some((key, val)) = line.split_once('=') {
                    if !val.is_empty() {
                        println!("export {}=\"{}\"", key.trim(), val.trim());
                    }
                }
            }
            println!();
        }
    }

    Ok(())
}

pub enum SecretsAction {
    List,
    Get(String),
    Set(String, String),
    Unset(String),
    Export,
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
