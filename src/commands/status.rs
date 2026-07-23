use crate::core::{config, filesystem, project};
use crate::error::HiveResult;
use crate::utils::terminal::*;
use colored::*;

pub fn run() -> HiveResult<()> {
    let cwd = project::current_dir();
    let cfg = config::load(&cwd)?;

    println!();
    print_step("◈", &format!("Project: {}", cfg.project.name));
    println!();
    print_divider();
    println!();

    print_label("       Name", &cfg.project.name);
    if let Some(desc) = &cfg.project.description {
        print_label("Description", desc);
    }
    if let Some(author) = &cfg.project.author {
        print_label("     Author", author);
    }
    if let Some(fw) = &cfg.project.framework {
        print_label("  Framework", fw);
    }

    println!();

    let env_path = filesystem::env_path(&cwd);
    let secrets_path = filesystem::secrets_path(&cwd);
    let log_path = filesystem::run_log_path(&cwd);

    let env_status = if env_path.exists() {
        "✦  present".truecolor(100, 200, 100).to_string()
    } else {
        "✗  missing".truecolor(200, 80, 80).to_string()
    };
    let secrets_status = if secrets_path.exists() {
        "✦  present".truecolor(100, 200, 100).to_string()
    } else {
        "✗  missing".truecolor(200, 80, 80).to_string()
    };
    let log_status = if log_path.exists() {
        let meta = std::fs::metadata(&log_path).ok();
        let size = meta
            .map(|m| format!("{} bytes", m.len()))
            .unwrap_or_default();
        format!("✦  {}", size).truecolor(100, 200, 100).to_string()
    } else {
        "—  no runs yet".truecolor(100, 100, 100).to_string()
    };

    print!("  {}  ", ".env".truecolor(180, 180, 180));
    println!("{}", env_status);
    print!("  {}  ", ".secrets".truecolor(180, 180, 180));
    println!("{}", secrets_status);
    print!("  {}  ", "run.log".truecolor(180, 180, 180));
    println!("{}", log_status);

    if !cfg.scripts.is_empty() {
        let scripts = &cfg.scripts;
        println!();
        print_label("  Scripts", "");
        for (name, _) in scripts {
            println!(
                "    {}  {}",
                "→".truecolor(255, 200, 0),
                name.truecolor(220, 220, 220)
            );
        }
    }

    if !cfg.services.is_empty() {
        let services = &cfg.services;
        println!();
        print_label("  Services", "");
        for (name, svc) in services {
            let port = svc.port.map(|p| format!(":{}", p)).unwrap_or_default();
            let cmd = svc.command.as_deref().unwrap_or("");
            println!(
                "    {}  {}{}  {}",
                "⬡".truecolor(147, 197, 253),
                name.truecolor(255, 200, 0).bold(),
                port.truecolor(100, 100, 100),
                cmd.truecolor(120, 120, 120)
            );
        }
    }

    println!();
    print_divider();
    println!();
    Ok(())
}
