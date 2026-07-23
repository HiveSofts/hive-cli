use crate::core::{config, filesystem, project};
use crate::error::{HiveError, HiveResult};
use crate::models::config::RunScript;
use crate::utils::terminal::*;
use colored::*;

pub enum ScriptsAction {
    List,
    Add(String, String),
    Remove(String),
    Show(String),
}

pub fn run(action: ScriptsAction) -> HiveResult<()> {
    let cwd = project::current_dir();

    if !filesystem::hive_initialized(&cwd) {
        return Err(HiveError::Config(
            "No .hive/config.yml found. Run `hive init` first.".into(),
        ));
    }

    match action {
        ScriptsAction::List => {
            let cfg = config::load(&cwd)?;
            println!();
            print_step("◈", "Scripts");
            println!();
            if cfg.scripts.is_empty() {
                print_warn("No scripts defined.");
            } else {
                for (name, script) in &cfg.scripts {
                    match script {
                        RunScript::Single(cmd) => {
                            println!(
                                "  {}  {}",
                                name.truecolor(255, 200, 0).bold(),
                                cmd.truecolor(120, 120, 120)
                            );
                        }
                        RunScript::Multiple(cmds) => {
                            println!(
                                "  {}  {}",
                                name.truecolor(255, 200, 0).bold(),
                                format!("[{} commands]", cmds.len()).truecolor(100, 100, 100)
                            );
                            for cmd in cmds {
                                println!(
                                    "    {}  {}",
                                    "·".truecolor(60, 60, 60),
                                    cmd.truecolor(100, 100, 100)
                                );
                            }
                        }
                    }
                }
            }
            println!();
        }

        ScriptsAction::Add(name, command) => {
            let mut cfg = config::load(&cwd)?;
            cfg.scripts
                .insert(name.clone(), RunScript::Single(command.clone()));
            config::save(&cwd, &cfg)?;
            print_success(&format!(
                "Added script '{}' → {}",
                name.truecolor(255, 200, 0).to_string().as_str(),
                command
            ));
        }

        ScriptsAction::Remove(name) => {
            let mut cfg = config::load(&cwd)?;
            if cfg.scripts.shift_remove(&name).is_none() {
                return Err(HiveError::Config(format!("Script '{}' not found", name)));
            }
            config::save(&cwd, &cfg)?;
            print_success(&format!("Removed script '{}'", name));
        }

        ScriptsAction::Show(name) => {
            let cfg = config::load(&cwd)?;
            let script = cfg
                .scripts
                .get(&name)
                .ok_or_else(|| HiveError::Config(format!("Script '{}' not found", name)))?;
            match script {
                RunScript::Single(cmd) => println!("{}", cmd),
                RunScript::Multiple(cmds) => {
                    for cmd in cmds {
                        println!("{}", cmd);
                    }
                }
            }
        }
    }

    Ok(())
}
