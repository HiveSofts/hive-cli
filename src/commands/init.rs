use std::thread;
use std::time::Duration;

use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use indexmap::IndexMap;

use crate::core::{config, filesystem, project, registry};
use crate::error::HiveResult;
use crate::models::config::{HiveConfig, ProjectConfig, RunScript, ServiceConfig};
use crate::services::detector;
use crate::utils::terminal::*;

pub fn run() -> HiveResult<()> {
    print_banner();

    registry::warn_if_no_global_hive();

    let cwd = project::current_dir();

    if filesystem::hive_initialized(&cwd) {
        print_warn("Project already initialized.");
        print_label("Location", &cwd.join(".hive").display().to_string());
        println!();
        return Ok(());
    }

    print_step("◈", "Scanning project...");
    thread::sleep(Duration::from_millis(350));

    let detected = project::detect(&cwd);
    let icon = detector::framework_icon(&detected.framework);
    let port = detector::default_port(&detected.framework);
    let cmd = detector::default_run_command(&detected.framework);
    let meta = &detected.meta;

    println!();
    print_divider();
    println!();

    print_label("  Project", &detected.name);
    if let Some(ver) = &meta.version {
        print_label("  Version", ver);
    }
    if let Some(desc) = &meta.description {
        print_label("     Info", desc);
    }
    print_label("Framework", &format!("{}  {}", icon, detected.framework));
    print_label("     Port", &port.to_string());
    if !meta.extra_services.is_empty() {
        print_label(" Services", &meta.extra_services.join("  ·  "));
    }

    println!();
    print_divider();
    println!();

    let theme = ColorfulTheme::default();

    let project_name: String = Input::with_theme(&theme)
        .with_prompt("  Project name")
        .default(detected.name.clone())
        .interact_text()
        .unwrap_or(detected.name.clone());

    let description: String = Input::with_theme(&theme)
        .with_prompt("  Description")
        .default(meta.description.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .unwrap_or_default();

    let author: String = Input::with_theme(&theme)
        .with_prompt("  Author")
        .allow_empty(true)
        .interact_text()
        .unwrap_or_default();

    let app_port: String = Input::with_theme(&theme)
        .with_prompt("  Port")
        .default(port.to_string())
        .interact_text()
        .unwrap_or(port.to_string());
    let app_port: u16 = app_port.parse().unwrap_or(port);

    let run_cmd: String = Input::with_theme(&theme)
        .with_prompt("  Run command")
        .default(cmd.to_string())
        .interact_text()
        .unwrap_or(cmd.to_string());

    println!();

    let extra_scripts_opts = vec![
        "dev  (all services together)",
        "build",
        "test",
        "migrate",
        "seed",
        "queue",
    ];
    let selected = MultiSelect::with_theme(&theme)
        .with_prompt("  Add script presets (space to select, enter to confirm)")
        .items(&extra_scripts_opts)
        .interact()
        .unwrap_or_default();

    println!();
    print_divider();
    println!();

    let sp = new_spinner("Creating .hive directory...");
    thread::sleep(Duration::from_millis(400));
    filesystem::create_hive_dir(&cwd)?;
    finish_spinner(&sp, ".hive/ created");

    let sp = new_spinner("Writing config...");
    thread::sleep(Duration::from_millis(350));

    let mut services = IndexMap::new();
    services.insert(
        "app".to_string(),
        ServiceConfig {
            port: Some(app_port),
            command: Some(run_cmd.clone()),
        },
    );
    if meta.has_docker {
        services.insert(
            "docker".to_string(),
            ServiceConfig {
                port: None,
                command: Some("docker compose up -d".to_string()),
            },
        );
    }

    let mut scripts: IndexMap<String, RunScript> = IndexMap::new();
    scripts.insert("start".to_string(), RunScript::Single(run_cmd.clone()));

    for idx in selected {
        match idx {
            0 => {
                let mut dev_cmds = vec![run_cmd.clone()];
                if meta.has_docker {
                    dev_cmds.push("docker compose up".to_string());
                }
                scripts.insert("dev".to_string(), RunScript::Multiple(dev_cmds));
            }
            1 => {
                scripts.insert(
                    "build".to_string(),
                    RunScript::Single("npm run build".to_string()),
                );
            }
            2 => {
                scripts.insert(
                    "test".to_string(),
                    RunScript::Single("npm test".to_string()),
                );
            }
            3 => {
                scripts.insert(
                    "migrate".to_string(),
                    RunScript::Single("php artisan migrate".to_string()),
                );
            }
            4 => {
                scripts.insert(
                    "seed".to_string(),
                    RunScript::Single("php artisan db:seed".to_string()),
                );
            }
            5 => {
                scripts.insert(
                    "queue".to_string(),
                    RunScript::Single("php artisan queue:listen".to_string()),
                );
            }
            _ => {}
        }
    }

    let cfg = HiveConfig {
        project: ProjectConfig {
            name: project_name.clone(),
            description: if description.is_empty() {
                None
            } else {
                Some(description.clone())
            },
            author: if author.is_empty() {
                None
            } else {
                Some(author.clone())
            },
            framework: Some(detected.framework.to_string()),
        },
        services,
        scripts,
    };
    config::save(&cwd, &cfg)?;
    finish_spinner(&sp, ".hive/config.yml written");

    let sp = new_spinner("Writing .env & .secrets...");
    thread::sleep(Duration::from_millis(300));

    let env_content =
        build_env_content(&project_name, app_port, meta.has_env, &description, &author);
    filesystem::write_file(&filesystem::env_path(&cwd), &env_content)?;

    let secrets_content = build_secrets_content(&project_name);
    filesystem::write_file(&filesystem::secrets_path(&cwd), &secrets_content)?;

    finish_spinner(&sp, ".hive/.env and .hive/.secrets written");

    // .gitignore auto-patch
    let sp = new_spinner("Patching .gitignore...");
    thread::sleep(Duration::from_millis(200));
    let gitignore_result = patch_gitignore(&cwd);
    match gitignore_result {
        Ok(patched) => {
            if patched {
                finish_spinner(&sp, ".gitignore updated — .hive/.secrets protected");
            } else {
                finish_spinner(&sp, ".gitignore already up to date");
            }
        }
        Err(_) => {
            finish_spinner(&sp, ".gitignore patch skipped");
        }
    }

    let sp = new_spinner("Registering project...");
    thread::sleep(Duration::from_millis(200));
    registry::register_project(&cwd, &cfg, Some(app_port))?;
    finish_spinner(&sp, "Project registered in ~/.hive/.cli/projects/");

    println!();
    print_divider();
    println!();
    print_success(&format!("Hive initialized  ·  {} {}", icon, project_name));
    println!();
    print_label(
        "   Config",
        &filesystem::config_path(&cwd).display().to_string(),
    );
    print_label(
        "      Env",
        &filesystem::env_path(&cwd).display().to_string(),
    );
    print_label(
        "  Secrets",
        &filesystem::secrets_path(&cwd).display().to_string(),
    );
    print_label(
        "     Logs",
        &filesystem::run_log_path(&cwd).display().to_string(),
    );
    println!();
    print_step("→", "`hive run`  —  run the default start script");
    print_step("→", "`hive run dev`  —  run the dev script");
    print_step("→", "`hive run --list`  —  show all scripts");
    print_step("→", "`hive env`  —  view environment variables");
    print_step("→", "`hive secrets`  —  manage secrets");
    print_step("→", "`hive projects`  —  list all registered projects");
    if meta.has_docker {
        print_step("→", "`hive run docker`  —  bring up compose services");
    }
    print_step("→", "`hive expose`  —  create a public tunnel");
    print_step("→", "`hive doctor`  —  check required tools");
    print_done();
    Ok(())
}

/// Returns Ok(true) if .gitignore was modified, Ok(false) if already correct, Err on IO failure.
fn patch_gitignore(project_root: &std::path::Path) -> std::io::Result<bool> {
    let gitignore_path = project_root.join(".gitignore");

    let hive_entries = [".hive/.secrets", ".hive/logs/"];

    let existing = if gitignore_path.exists() {
        std::fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    let mut missing: Vec<&str> = hive_entries
        .iter()
        .filter(|entry| !existing.lines().any(|l| l.trim() == **entry))
        .copied()
        .collect();

    if missing.is_empty() {
        return Ok(false);
    }

    let mut content = existing;

    // ensure trailing newline before appending
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }

    content.push_str("\n# Hive CLI\n");
    for entry in &missing {
        content.push_str(entry);
        content.push('\n');
    }

    std::fs::write(&gitignore_path, content)?;
    Ok(true)
}

fn build_env_content(
    name: &str,
    port: u16,
    has_existing: bool,
    desc: &str,
    author: &str,
) -> String {
    let note = if has_existing {
        "# Synced from your existing .env — add overrides below"
    } else {
        "# Hive environment variables — edit freely"
    };
    let mut lines = vec![
        note.to_string(),
        String::new(),
        format!("APP_NAME={}", name),
        "APP_ENV=development".to_string(),
        format!("APP_PORT={}", port),
        "APP_DEBUG=true".to_string(),
        format!("APP_URL=http://localhost:{}", port),
    ];
    if !desc.is_empty() {
        lines.push(format!("APP_DESCRIPTION={}", desc));
    }
    if !author.is_empty() {
        lines.push(format!("APP_AUTHOR={}", author));
    }
    lines.push(String::new());
    lines.push("# Database".to_string());
    lines.push("DB_CONNECTION=mysql".to_string());
    lines.push("DB_HOST=127.0.0.1".to_string());
    lines.push("DB_PORT=3306".to_string());
    lines.push(format!(
        "DB_DATABASE={}",
        name.to_lowercase().replace(' ', "_")
    ));
    lines.push("DB_USERNAME=root".to_string());
    lines.push("DB_PASSWORD=".to_string());
    lines.push(String::new());
    lines.push("# Cache / Queue".to_string());
    lines.push("CACHE_DRIVER=file".to_string());
    lines.push("QUEUE_CONNECTION=sync".to_string());
    lines.push("SESSION_DRIVER=file".to_string());
    lines.join("\n") + "\n"
}

fn build_secrets_content(name: &str) -> String {
    format!(
        "# Hive Secrets — {}\n\
         # ⚠  Do NOT commit this file\n\
         # Add sensitive values here; access via `hive secrets get KEY`\n\
         #\n\
         # Example:\n\
         #   JWT_SECRET=your-super-secret-key\n\
         #   STRIPE_KEY=sk_live_...\n\
         #   SMTP_PASSWORD=hunter2\n\
         #\n\
         \n\
         JWT_SECRET=\n\
         API_KEY=\n\
         STRIPE_SECRET=\n\
         SMTP_HOST=\n\
         SMTP_PORT=587\n\
         SMTP_USER=\n\
         SMTP_PASS=\n\
         ",
        name
    )
}
