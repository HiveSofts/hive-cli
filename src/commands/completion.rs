use crate::cli::commands::Cli;
use crate::error::{HiveError, HiveResult};
use crate::utils::terminal::print_step;
use clap::CommandFactory;
use clap_complete::{generate, shells};
use std::io;

pub fn run(shell: &str) -> HiveResult<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();

    match shell.to_lowercase().as_str() {
        "bash" => {
            generate(shells::Bash, &mut cmd, name, &mut io::stdout());
        }
        "zsh" => {
            generate(shells::Zsh, &mut cmd, name, &mut io::stdout());
        }
        "fish" => {
            generate(shells::Fish, &mut cmd, name, &mut io::stdout());
        }
        "powershell" | "ps" => {
            generate(shells::PowerShell, &mut cmd, name, &mut io::stdout());
        }
        other => {
            return Err(HiveError::Config(format!(
                "Unknown shell '{}'. Supported: bash, zsh, fish, powershell",
                other
            )));
        }
    }

    Ok(())
}

pub fn print_install_hint(shell: &str) {
    match shell.to_lowercase().as_str() {
        "bash" => {
            print_step("→", "Add to ~/.bashrc:");
            print_step(" ", "  eval \"$(hive completion bash)\"");
        }
        "zsh" => {
            print_step("→", "Add to ~/.zshrc:");
            print_step(" ", "  eval \"$(hive completion zsh)\"");
        }
        "fish" => {
            print_step("→", "Run:");
            print_step(
                " ",
                "  hive completion fish > ~/.config/fish/completions/hive.fish",
            );
        }
        _ => {}
    }
}
