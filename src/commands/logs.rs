use crate::core::filesystem;
use crate::core::project;
use crate::error::{HiveError, HiveResult};
use crate::utils::terminal::*;
use colored::*;

pub fn run(lines: Option<usize>, follow: bool) -> HiveResult<()> {
    let cwd = project::current_dir();
    let log_path = filesystem::run_log_path(&cwd);

    if !log_path.exists() {
        return Err(HiveError::Config(
            "No run.log found. Run `hive run` first.".into(),
        ));
    }

    let n = lines.unwrap_or(50);

    println!();
    print_step("📋", &format!("Logs  —  {}", log_path.display()));
    println!();
    print_divider();
    println!();

    let content = std::fs::read_to_string(&log_path)?;
    let all_lines: Vec<&str> = content.lines().collect();
    let start = if all_lines.len() > n {
        all_lines.len() - n
    } else {
        0
    };

    for line in &all_lines[start..] {
        print_log_line(line);
    }

    if follow {
        println!();
        print_step("⟳", "Following log... (Ctrl+C to stop)");
        println!();

        let mut pos = content.len();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(150));
            match std::fs::read_to_string(&log_path) {
                Ok(new_content) if new_content.len() > pos => {
                    let new_part = &new_content[pos..];
                    for line in new_part.lines() {
                        print_log_line(line);
                    }
                    pos = new_content.len();
                }
                _ => {}
            }
        }
    }

    println!();
    Ok(())
}

fn print_log_line(line: &str) {
    if line.contains("[stderr]") {
        println!("  {}", line.truecolor(220, 130, 130));
    } else if line.starts_with('╔') || line.starts_with('╚') || line.starts_with('║') {
        println!("  {}", line.truecolor(255, 200, 0).bold());
    } else if line.contains("ERROR") || line.contains("error") {
        println!("  {}", line.truecolor(220, 80, 80));
    } else if line.contains("WARN") || line.contains("warn") {
        println!("  {}", line.truecolor(255, 160, 0));
    } else {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 3 {
            let ts = parts[0..2].join(" ");
            let rest = parts[2..].join(" ");
            print!("  {}", ts.truecolor(70, 70, 70));
            if rest.starts_with('[') {
                if let Some(end) = rest.find(']') {
                    let label = &rest[..=end];
                    let msg = &rest[end + 1..];
                    print!("  {}", label.truecolor(147, 197, 253).bold());
                    println!("{}", msg.truecolor(200, 200, 200));
                    return;
                }
            }
            println!("  {}", rest.truecolor(200, 200, 200));
        } else {
            println!("  {}", line.truecolor(160, 160, 160));
        }
    }
}
