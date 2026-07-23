use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "hive",
    about = "🐝 Hive — your local dev environment, orchestrated",
    version = "0.1.0",
    disable_help_subcommand = true,
    disable_version_flag = false
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize Hive in the current project")]
    Init,

    #[command(about = "Run a script from .hive/config.yml (default: start)")]
    Run {
        #[arg(help = "Script name to run (e.g. dev, build, test)")]
        script: Option<String>,

        #[arg(long, help = "List available scripts")]
        list: bool,

        #[arg(long, help = "Show what would run without executing")]
        dry_run: bool,
    },

    #[command(about = "View and tail run logs")]
    Logs {
        #[arg(short = 'n', long, help = "Number of lines to show (default: 50)")]
        lines: Option<usize>,

        #[arg(short = 'f', long, help = "Follow log output")]
        follow: bool,
    },

    #[command(about = "Manage environment variables (.hive/.env)")]
    Env {
        #[command(subcommand)]
        action: Option<EnvCommands>,
    },

    #[command(about = "Manage secrets (.hive/.secrets)")]
    Secrets {
        #[command(subcommand)]
        action: Option<SecretsCommands>,
    },

    #[command(about = "Expose a local port via tunnel")]
    Expose {
        #[arg(help = "Local port to expose")]
        port: u16,
    },

    #[command(about = "Show project status and config")]
    Status,

    #[command(about = "Manage registered projects")]
    Projects {
        #[command(subcommand)]
        action: Option<ProjectsCommands>,
    },

    #[command(about = "Jump to a registered project directory: eval $(hive cd <name>)")]
    Cd {
        #[arg(help = "Project name or partial match")]
        query: String,
    },

    #[command(about = "Manage scripts in .hive/config.yml")]
    Scripts {
        #[command(subcommand)]
        action: Option<ScriptsCommands>,
    },

    #[command(about = "View or edit .hive/config.yml")]
    Config {
        #[command(subcommand)]
        action: Option<ConfigCommands>,
    },

    #[command(about = "Check that required tools are installed")]
    Doctor,

    #[command(about = "Generate shell completion scripts")]
    Completion {
        #[arg(help = "Shell: bash, zsh, fish, powershell")]
        shell: String,
    },
}

#[derive(Subcommand)]
pub enum EnvCommands {
    #[command(about = "List all env vars")]
    List,
    #[command(about = "Get a value: hive env get KEY")]
    Get { key: String },
    #[command(about = "Set a value: hive env set KEY VALUE")]
    Set { key: String, value: String },
    #[command(about = "Remove a key: hive env unset KEY")]
    Unset { key: String },
}

#[derive(Subcommand)]
pub enum SecretsCommands {
    #[command(about = "List all secrets (masked)")]
    List,
    #[command(about = "Get a secret value: hive secrets get KEY")]
    Get { key: String },
    #[command(about = "Set a secret: hive secrets set KEY VALUE")]
    Set { key: String, value: String },
    #[command(about = "Remove a secret: hive secrets unset KEY")]
    Unset { key: String },
    #[command(about = "Export secrets as shell export statements")]
    Export,
}

#[derive(Subcommand)]
pub enum ProjectsCommands {
    #[command(about = "List all registered projects")]
    List,
    #[command(about = "Print cd command for a project: eval $(hive projects cd myapp)")]
    Cd { query: String },
    #[command(about = "Show project details")]
    Info { query: String },
}

#[derive(Subcommand)]
pub enum ScriptsCommands {
    #[command(about = "List all scripts")]
    List,
    #[command(about = "Add a script: hive scripts add NAME COMMAND")]
    Add { name: String, command: String },
    #[command(about = "Remove a script: hive scripts remove NAME")]
    Remove { name: String },
    #[command(about = "Show a script's command: hive scripts show NAME")]
    Show { name: String },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Show current config")]
    Show,
    #[command(about = "Set a config value: hive config set project.name MyApp")]
    Set { key: String, value: String },
}
