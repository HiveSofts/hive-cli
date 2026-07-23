use crate::core::registry;
use crate::error::{HiveError, HiveResult};
use crate::utils::terminal::*;
use colored::*;

pub fn run(action: ProjectsAction) -> HiveResult<()> {
    match action {
        ProjectsAction::List => list(),
        ProjectsAction::Cd(query) => cd(&query),
        ProjectsAction::Info(query) => info(&query),
    }
}

pub enum ProjectsAction {
    List,
    Cd(String),
    Info(String),
}

fn list() -> HiveResult<()> {
    let global_exists = registry::global_hive_exists();

    println!();
    print_step("◈", "Registered Projects");
    println!();

    let mut all: Vec<_> = vec![];

    if global_exists {
        match registry::get_hive_app_projects() {
            Ok(projects) if !projects.is_empty() => {
                print_label("  Source", "~/.hive/projects/  (Hive App)");
                println!();
                for p in &projects {
                    let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                    let path = p.get("path").and_then(|v| v.as_str()).unwrap_or("?");
                    let kind = p.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    let port = p
                        .get("port")
                        .and_then(|v| v.as_u64())
                        .map(|p| format!(":{}", p))
                        .unwrap_or_default();
                    println!(
                        "  {}  {}  {}  {}",
                        "◆".truecolor(255, 200, 0),
                        name.truecolor(255, 200, 0).bold(),
                        format!("[{}{}]", kind, port).truecolor(100, 100, 100),
                        path.truecolor(80, 80, 80)
                    );
                    all.push(name.to_string());
                }
                println!();
            }
            _ => {}
        }
    }

    match registry::list_projects() {
        Ok(projects) if !projects.is_empty() => {
            print_label("  Source", "~/.hive/.cli/projects/  (CLI)");
            println!();
            for (_, entry) in &projects {
                let port = entry.port.map(|p| format!(":{}", p)).unwrap_or_default();
                let framework = entry.framework.as_deref().unwrap_or("?");
                let desc = entry.description.as_deref().unwrap_or("");
                println!(
                    "  {}  {}  {}{}",
                    "⬡".truecolor(147, 197, 253),
                    entry.name.truecolor(255, 200, 0).bold(),
                    format!("[{}{}]", framework, port).truecolor(100, 100, 100),
                    if desc.is_empty() {
                        String::new()
                    } else {
                        format!("  {}", desc.truecolor(120, 120, 120))
                    }
                );
                println!("       {}", entry.path.truecolor(70, 70, 70));
                all.push(entry.name.clone());
            }
            println!();
        }
        Ok(_) => {
            print_warn("No CLI-registered projects found.");
            print_step(
                "→",
                "Run `hive init` in a project directory to register it.",
            );
            println!();
        }
        Err(e) => {
            print_warn(&format!("Could not read CLI projects: {}", e));
        }
    }

    if all.is_empty() {
        print_warn("No projects found in any source.");
    } else {
        print_divider();
        println!();
        print_step(
            "→",
            "`hive projects cd <name>`  —  print project path (use with cd)",
        );
        print_step("→", "`hive projects info <name>`  —  view project details");
        print_step("→", "`hive cd <name>`  —  shortcut: eval $(hive cd myapp)");
        println!();
        println!(
            "  {}",
            "Tip: eval $(hive cd myapp) to jump directories".truecolor(80, 80, 80)
        );
        println!();
    }

    Ok(())
}

fn cd(query: &str) -> HiveResult<()> {
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

fn info(query: &str) -> HiveResult<()> {
    if let Some(entry) = registry::find_project(query)? {
        println!();
        print_step("◈", &format!("Project: {}", entry.name));
        println!();
        print_label("       Name", &entry.name);
        if let Some(desc) = &entry.description {
            print_label("Description", desc);
        }
        if let Some(author) = &entry.author {
            print_label("     Author", author);
        }
        if let Some(fw) = &entry.framework {
            print_label("  Framework", fw);
        }
        if let Some(port) = entry.port {
            print_label("       Port", &port.to_string());
        }
        print_label("       Path", &entry.path);
        print_label("    Created", &entry.created_at);
        println!();
        return Ok(());
    }

    if registry::global_hive_exists() {
        if let Ok(projects) = registry::get_hive_app_projects() {
            for p in &projects {
                let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("");
                if name.to_lowercase().contains(&query.to_lowercase()) {
                    println!();
                    print_step("◈", &format!("Project: {}  (Hive App)", name));
                    println!();
                    for (key, val) in p.as_object().unwrap() {
                        let val_str = match val {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            _ => val.to_string(),
                        };
                        print_label(&format!("{:>15}", key), &val_str);
                    }
                    println!();
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
