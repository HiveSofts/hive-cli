mod cli;
mod commands;
mod core;
mod error;
mod models;
mod services;
mod utils;

use clap::Parser;
use cli::commands::{
    Cli, Commands, ConfigCommands, EnvCommands, ProjectsCommands, ScriptsCommands, SecretsCommands,
};
use commands::env::EnvAction;
use commands::hive_config::ConfigAction;
use commands::projects::ProjectsAction;
use commands::scripts::ScriptsAction;
use commands::secrets::SecretsAction;
use utils::terminal::print_error;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => commands::init::run(),

        Commands::Run {
            script,
            list,
            dry_run,
        } => commands::run::run(script, list, dry_run),

        Commands::Logs { lines, follow } => commands::logs::run(lines, follow),

        Commands::Status => commands::status::run(),

        Commands::Expose { port } => commands::expose::run(port),

        Commands::Doctor => commands::doctor::run(),

        Commands::Cd { query } => commands::cd::run(&query),

        Commands::Completion { shell } => commands::completion::run(&shell),

        Commands::Env { action } => {
            let action = match action {
                None | Some(EnvCommands::List) => EnvAction::List,
                Some(EnvCommands::Get { key }) => EnvAction::Get(key),
                Some(EnvCommands::Set { key, value }) => EnvAction::Set(key, value),
                Some(EnvCommands::Unset { key }) => EnvAction::Unset(key),
            };
            commands::env::run(action)
        }

        Commands::Secrets { action } => {
            let action = match action {
                None | Some(SecretsCommands::List) => SecretsAction::List,
                Some(SecretsCommands::Get { key }) => SecretsAction::Get(key),
                Some(SecretsCommands::Set { key, value }) => SecretsAction::Set(key, value),
                Some(SecretsCommands::Unset { key }) => SecretsAction::Unset(key),
                Some(SecretsCommands::Export) => SecretsAction::Export,
            };
            commands::secrets::run(action)
        }

        Commands::Projects { action } => {
            let action = match action {
                None | Some(ProjectsCommands::List) => ProjectsAction::List,
                Some(ProjectsCommands::Cd { query }) => ProjectsAction::Cd(query),
                Some(ProjectsCommands::Info { query }) => ProjectsAction::Info(query),
            };
            commands::projects::run(action)
        }

        Commands::Scripts { action } => {
            let action = match action {
                None | Some(ScriptsCommands::List) => ScriptsAction::List,
                Some(ScriptsCommands::Add { name, command }) => ScriptsAction::Add(name, command),
                Some(ScriptsCommands::Remove { name }) => ScriptsAction::Remove(name),
                Some(ScriptsCommands::Show { name }) => ScriptsAction::Show(name),
            };
            commands::scripts::run(action)
        }

        Commands::Config { action } => {
            let action = match action {
                None | Some(ConfigCommands::Show) => ConfigAction::Show,
                Some(ConfigCommands::Set { key, value }) => ConfigAction::Set(key, value),
            };
            commands::hive_config::run(action)
        }
    };

    if let Err(e) = result {
        print_error(&e.to_string());
        std::process::exit(1);
    }
}
