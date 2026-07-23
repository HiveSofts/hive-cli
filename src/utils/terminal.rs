use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn print_banner() {
    println!();
    println!(
        "{}",
        "  ██╗  ██╗██╗██╗   ██╗███████╗"
            .truecolor(255, 200, 0)
            .bold()
    );
    println!(
        "{}",
        "  ██║  ██║██║██║   ██║██╔════╝"
            .truecolor(255, 200, 0)
            .bold()
    );
    println!(
        "{}",
        "  ███████║██║██║   ██║█████╗  "
            .truecolor(255, 180, 0)
            .bold()
    );
    println!(
        "{}",
        "  ██╔══██║██║╚██╗ ██╔╝██╔══╝  "
            .truecolor(255, 160, 0)
            .bold()
    );
    println!(
        "{}",
        "  ██║  ██║██║ ╚████╔╝ ███████╗"
            .truecolor(255, 140, 0)
            .bold()
    );
    println!(
        "{}",
        "  ╚═╝  ╚═╝╚═╝  ╚═══╝  ╚══════╝"
            .truecolor(255, 120, 0)
            .bold()
    );
    println!();
    println!(
        "  {} {}  {}",
        "🐝",
        "Hive CLI".truecolor(255, 200, 0).bold(),
        "v0.1.0".truecolor(80, 80, 80)
    );
    println!(
        "  {}",
        "Your local dev environment, orchestrated.".truecolor(120, 120, 120)
    );
    println!();
    println!(
        "  {}",
        "─────────────────────────────────────────".truecolor(50, 50, 50)
    );
    println!();
}

pub fn print_step(icon: &str, msg: &str) {
    println!(
        "  {} {}",
        icon.truecolor(255, 200, 0),
        msg.truecolor(220, 220, 220)
    );
}

pub fn print_success(msg: &str) {
    println!(
        "  {} {}",
        "✦".truecolor(100, 220, 120).bold(),
        msg.truecolor(200, 220, 200)
    );
}

pub fn print_error(msg: &str) {
    eprintln!(
        "  {} {}",
        "✗".truecolor(220, 60, 60).bold(),
        msg.truecolor(220, 100, 100)
    );
}

pub fn print_warn(msg: &str) {
    println!(
        "  {} {}",
        "◆".truecolor(255, 160, 0).bold(),
        msg.truecolor(200, 160, 80)
    );
}

pub fn print_label(label: &str, value: &str) {
    println!(
        "  {}  {}",
        label.truecolor(90, 90, 90),
        value.truecolor(255, 200, 0).bold()
    );
}

pub fn print_label_plain(label: &str, value: &str) {
    println!(
        "  {}  {}",
        label.truecolor(90, 90, 90),
        value.truecolor(200, 200, 200)
    );
}

pub fn print_divider() {
    println!(
        "  {}",
        "─────────────────────────────────────────".truecolor(45, 45, 45)
    );
}

pub fn print_done() {
    println!();
    println!("  {} {}", "🐝", "Done!".truecolor(255, 200, 0).bold());
    println!();
}

pub fn new_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template(&format!(
            "  {{spinner:.yellow}} {}",
            msg.truecolor(160, 160, 160)
        ))
        .unwrap()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

pub fn finish_spinner(pb: &ProgressBar, msg: &str) {
    pb.finish_and_clear();
    print_success(msg);
}

pub fn check_mark(ok: bool) -> ColoredString {
    if ok {
        "✦".truecolor(100, 220, 120).bold()
    } else {
        "✗".truecolor(220, 60, 60).bold()
    }
}

pub fn status_ok(msg: &str) -> String {
    format!(
        "{}  {}",
        "✦".truecolor(100, 220, 120).bold(),
        msg.truecolor(160, 220, 160)
    )
}

pub fn status_err(msg: &str) -> String {
    format!(
        "{}  {}",
        "✗".truecolor(220, 60, 60).bold(),
        msg.truecolor(220, 100, 100)
    )
}

pub fn status_warn(msg: &str) -> String {
    format!(
        "{}  {}",
        "◆".truecolor(255, 160, 0).bold(),
        msg.truecolor(200, 160, 80)
    )
}
