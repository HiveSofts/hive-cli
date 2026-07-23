use crate::core::{config, filesystem, project};
use crate::error::{HiveError, HiveResult};
use crate::models::config::RunScript;
use crate::services::process::ProcessGroup;
use crate::utils::terminal::*;
use chrono::Local;
use colored::Colorize;

pub fn run(script_name: Option<String>, list: bool, dry_run: bool) -> HiveResult<()> {
    let cwd = project::current_dir();

    if !filesystem::hive_initialized(&cwd) {
        return Err(HiveError::Config(
            "No .hive/config.yml found. Run `hive init` first.".into(),
        ));
    }

    let cfg = config::load(&cwd)?;
    let scripts = &cfg.scripts;

    if list || script_name.is_none() && scripts.is_empty() {
        println!();
        print_step("◈", "Available scripts");
        println!();
        if scripts.is_empty() {
            print_warn("No scripts defined in .hive/config.yml");
            print_step("→", "Add scripts under the `scripts:` key in your config.");
        } else {
            for (name, script) in scripts {
                match script {
                    RunScript::Single(cmd) => {
                        println!(
                            "  {}  {}  {}",
                            "→".truecolor(255, 200, 0),
                            name.truecolor(255, 200, 0).bold(),
                            cmd.truecolor(100, 100, 100)
                        );
                    }
                    RunScript::Multiple(cmds) => {
                        println!(
                            "  {}  {}  {}",
                            "→".truecolor(255, 200, 0),
                            name.truecolor(255, 200, 0).bold(),
                            format!("[{} commands]", cmds.len()).truecolor(100, 100, 100)
                        );
                        for cmd in cmds {
                            println!(
                                "       {}  {}",
                                "·".truecolor(60, 60, 60),
                                cmd.truecolor(80, 80, 80)
                            );
                        }
                    }
                }
            }
        }
        println!();
        return Ok(());
    }

    let script_key = script_name.as_deref().unwrap_or("start");

    if scripts.is_empty() {
        return Err(HiveError::Config(
            "No scripts defined in .hive/config.yml".into(),
        ));
    }

    let script = scripts.get(script_key).ok_or_else(|| {
        let available = scripts.keys().cloned().collect::<Vec<_>>().join(", ");
        HiveError::Config(format!(
            "Script '{}' not found.\n\n  Available: {}\n\n  Run `hive run --list` to see all scripts.",
            script_key, available
        ))
    })?;

    if dry_run {
        println!();
        print_step("◈", &format!("Dry run — script '{}'", script_key));
        println!();
        match script {
            RunScript::Single(cmd) => {
                print_label("  Command", cmd);
            }
            RunScript::Multiple(cmds) => {
                print_label("  Processes", &cmds.len().to_string());
                println!();
                for (i, cmd) in cmds.iter().enumerate() {
                    println!(
                        "  {}  {}",
                        format!("[{}]", i + 1).truecolor(100, 100, 100),
                        cmd.truecolor(220, 220, 220)
                    );
                }
            }
        }
        println!();
        return Ok(());
    }

    let log_path = filesystem::run_log_path(&cwd);
    if let Some(parent) = log_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!();
    print_step(
        "▶",
        &format!(
            "Running script  {}  [{}]",
            format!("'{}'", script_key)
                .truecolor(255, 200, 0)
                .to_string()
                .as_str(),
            ts
        ),
    );
    println!();
    print_label("  Log", &log_path.display().to_string());
    println!();
    print_divider();
    println!();

    let session_header = format!(
        "\n╔══════════════════════════════════════╗\n\
         ║  {} — script: {}  \n\
         ╚══════════════════════════════════════╝\n",
        ts, script_key
    );
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| {
            use std::io::Write;
            f.write_all(session_header.as_bytes())
        })?;

    match script {
        RunScript::Single(cmd) => {
            let pg = ProcessGroup::new(log_path.clone());
            pg.run_concurrent(vec![(script_key.to_string(), cmd.clone())])?;
        }
        RunScript::Multiple(cmds) => {
            if cmds.is_empty() {
                return Err(HiveError::Config(format!(
                    "Script '{}' has no commands",
                    script_key
                )));
            }
            print_step(
                "◈",
                &format!("Starting {} processes concurrently", cmds.len()),
            );
            println!();
            for (i, cmd) in cmds.iter().enumerate() {
                print_label(&format!("  [{}]", i + 1), cmd);
            }
            println!();
            print_divider();
            println!();

            let named: Vec<(String, String)> = cmds
                .iter()
                .enumerate()
                .map(|(i, cmd)| {
                    let label = cmd.split_whitespace().next().unwrap_or("proc").to_string();
                    let label = format!("{}:{}", label, i + 1);
                    (label, cmd.clone())
                })
                .collect();

            let pg = ProcessGroup::new(log_path.clone());
            pg.run_concurrent(named)?;
        }
    }

    println!();
    print_divider();
    println!();
    print_success(&format!("Script '{}' finished.", script_key));
    print_step("→", &format!("Log saved to: {}", log_path.display()));
    println!();
    Ok(())
}
