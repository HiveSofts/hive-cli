use crate::error::HiveResult;
use crate::utils::terminal::*;
use colored::*;
use std::process::Command;

struct Tool {
    name: &'static str,
    check: &'static str,
    version_flag: &'static str,
    required: bool,
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "node",
        check: "node",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "npm",
        check: "npm",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "bun",
        check: "bun",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "php",
        check: "php",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "composer",
        check: "composer",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "python3",
        check: "python3",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "pip",
        check: "pip",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "cargo",
        check: "cargo",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "go",
        check: "go",
        version_flag: "version",
        required: false,
    },
    Tool {
        name: "docker",
        check: "docker",
        version_flag: "--version",
        required: false,
    },
    Tool {
        name: "git",
        check: "git",
        version_flag: "--version",
        required: true,
    },
];

pub fn run() -> HiveResult<()> {
    println!();
    print_step("🩺", "Hive Doctor — checking environment");
    println!();
    print_divider();
    println!();

    let mut ok_count = 0;
    let mut missing_required = vec![];

    for tool in TOOLS {
        let result = which::which(tool.check);
        match result {
            Ok(path) => {
                let version = get_version(tool.check, tool.version_flag);
                let version_str = version.as_deref().unwrap_or("?");
                println!(
                    "  {}  {}  {}  {}",
                    "✦".truecolor(100, 220, 120).bold(),
                    tool.name.truecolor(220, 220, 220).bold(),
                    version_str.truecolor(100, 100, 100),
                    path.display().to_string().truecolor(60, 60, 60)
                );
                ok_count += 1;
            }
            Err(_) => {
                let label = if tool.required {
                    "(required)"
                } else {
                    "(optional)"
                };
                println!(
                    "  {}  {}  {}",
                    "✗".truecolor(180, 60, 60).bold(),
                    tool.name.truecolor(160, 160, 160),
                    label.truecolor(80, 80, 80)
                );
                if tool.required {
                    missing_required.push(tool.name);
                }
            }
        }
    }

    println!();
    print_divider();
    println!();

    if missing_required.is_empty() {
        print_success(&format!("{} tools found", ok_count));
    } else {
        print_warn(&format!(
            "Missing required tools: {}",
            missing_required.join(", ")
        ));
    }

    println!();
    Ok(())
}

fn get_version(cmd: &str, flag: &str) -> Option<String> {
    let out = Command::new(cmd).arg(flag).output().ok()?;
    let raw = String::from_utf8_lossy(&out.stdout);
    let line = raw.lines().next()?;
    // trim down to just the version number when possible
    let trimmed = line
        .split_whitespace()
        .find(|s| {
            s.chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        })
        .unwrap_or(line.trim());
    Some(trimmed.to_string())
}
