use crate::core::{config, filesystem, project};
use crate::error::{HiveError, HiveResult};
use crate::models::config::RunScript;
use crate::services::process::ProcessGroup;
use crate::utils::terminal::*;
use chrono::Local;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

fn hive_home() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".hive")
}

fn cloudflared_bin_name() -> &'static str {
    if cfg!(windows) {
        "cloudflared.exe"
    } else {
        "cloudflared"
    }
}

/// Returns the path to use for cloudflared (may not exist yet).
/// Priority:
///   1. ~/.hive/bin/cloudflared          — exists → use it
///   2. ~/.hive/.cli/bin/cloudflared     — install here if needed
fn resolve_cloudflared() -> (PathBuf, bool) {
    let primary = hive_home().join("bin").join(cloudflared_bin_name());
    if primary.exists() {
        return (primary, true);
    }
    let secondary = hive_home()
        .join(".cli")
        .join("bin")
        .join(cloudflared_bin_name());
    let exists = secondary.exists();
    (secondary, exists)
}

fn download_url() -> &'static str {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    match (os, arch) {
        ("windows", _)                              => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe",
        ("linux",  "x86_64")                        => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64",
        ("linux",  "x86") | ("linux", "i686")
        | ("linux", "i386")                         => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-386",
        ("linux",  "aarch64") | ("linux", "arm64")  => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64",
        ("linux",  "arm")                           => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm",
        ("macos",  "aarch64") | ("macos", "arm64")  => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-darwin-arm64.tgz",
        ("macos",  _)                               => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-darwin-amd64.tgz",
        _                                           => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64",
    }
}

#[cfg(unix)]
fn set_executable(path: &PathBuf) -> HiveResult<()> {
    use std::os::unix::fs::PermissionsExt;
    let meta = fs::metadata(path)?;
    let mut perm = meta.permissions();
    perm.set_mode(0o755);
    fs::set_permissions(path, perm)?;
    Ok(())
}

#[cfg(windows)]
fn set_executable(_path: &PathBuf) -> HiveResult<()> {
    Ok(())
}

fn install_cloudflared(dest: &PathBuf) -> HiveResult<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    let url = download_url();
    let is_tgz = url.ends_with(".tgz");

    println!();
    print_step(
        "⬇",
        &format!("Downloading cloudflared  {}", url.truecolor(70, 70, 70)),
    );
    println!();

    // We're in a sync context (CLI), so use blocking reqwest.
    // Cargo.toml already has reqwest for the tunnel feature; add features = ["blocking"] if needed.
    // If the project doesn't have reqwest yet, fall back to curl/wget subprocess.
    download_with_progress(url, dest, is_tgz)?;

    set_executable(dest)?;

    println!();
    print_success("cloudflared installed");
    Ok(())
}

/// Try ureq (pure Rust, no async) → fall back to curl → wget
fn download_with_progress(url: &str, dest: &PathBuf, is_tgz: bool) -> HiveResult<()> {
    // We'll use a subprocess (curl/wget) so we don't need a new dep.
    // Progress is faked with a spinner; for real byte progress we'd need reqwest.
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.yellow}  {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Connecting…");

    let write_path = if is_tgz {
        dest.parent().unwrap().join("cloudflared.tgz")
    } else {
        dest.clone()
    };

    // try curl first, then wget
    let curl_ok = std::process::Command::new("curl")
        .args([
            "-L",
            "--progress-bar",
            "-o",
            write_path.to_str().unwrap(),
            url,
        ])
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !curl_ok {
        pb.set_message("Trying wget…");
        let wget_ok = std::process::Command::new("wget")
            .args([
                "-q",
                "--show-progress",
                "-O",
                write_path.to_str().unwrap(),
                url,
            ])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !wget_ok {
            pb.finish_and_clear();
            return Err(HiveError::Process(
                "Could not download cloudflared. Install curl or wget, or download manually."
                    .into(),
            ));
        }
    }

    pb.set_message("Extracting…");

    if is_tgz {
        let status = std::process::Command::new("tar")
            .args([
                "-xzf",
                write_path.to_str().unwrap(),
                "-C",
                write_path.parent().unwrap().to_str().unwrap(),
            ])
            .status()
            .map_err(|e| HiveError::Process(e.to_string()))?;

        let _ = fs::remove_file(&write_path);

        if !status.success() {
            return Err(HiveError::Process(
                "Failed to extract cloudflared archive".into(),
            ));
        }

        // tgz extracts as just "cloudflared"
        let extracted = write_path.parent().unwrap().join("cloudflared");
        if extracted.exists() && &extracted != dest {
            fs::rename(&extracted, dest)?;
        }
    }

    pb.finish_and_clear();
    Ok(())
}

fn tunnel_log_dir(project_root: &PathBuf) -> PathBuf {
    filesystem::logs_dir(project_root).join("tunnels")
}

fn tunnel_log_path(project_root: &PathBuf) -> PathBuf {
    let ts = Local::now().format("%Y%m%d_%H%M%S");
    tunnel_log_dir(project_root).join(format!("tunnel_{}.log", ts))
}

fn write_tunnel_log(log: &PathBuf, line: &str, is_err: bool) {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(log) {
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level = if is_err { "ERR" } else { "INF" };
        let _ = writeln!(f, "[{}] [{}] {}", ts, level, line);
    }
}

fn extract_url(line: &str) -> Option<String> {
    // JSON format from cloudflared
    if line.trim_start().starts_with('{') {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(url) = v.get("url").and_then(|u| u.as_str()) {
                if url.starts_with("https://") {
                    return Some(url.to_string());
                }
            }
        }
    }
    // Plain text format
    if let Some(pos) = line.find("https://") {
        let rest = &line[pos..];
        let end = rest
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ',')
            .unwrap_or(rest.len());
        let url = &rest[..end];
        if url.contains('.')
            && (url.contains("trycloudflare.com") || url.contains("cfargotunnel.com"))
        {
            return Some(url.to_string());
        }
    }
    None
}

pub fn run(port: u16) -> HiveResult<()> {
    let cwd = project::current_dir();
    let cfg = config::load(&cwd)?;

    println!();
    print_step(
        "🌐",
        &format!(
            "Expose  —  {}",
            cfg.project.name.truecolor(255, 200, 0).bold()
        ),
    );
    println!();

    let (bin_path, already_installed) = resolve_cloudflared();

    if already_installed {
        print_success(&format!(
            "cloudflared found  {}",
            bin_path.display().to_string().truecolor(70, 70, 70)
        ));
    } else {
        print_warn("cloudflared not found — installing…");
        install_cloudflared(&bin_path)?;
    }

    let local_url = format!("http://localhost:{}", port);
    let service_running = check_port_open(port);

    if !service_running {
        println!();
        print_step("▶", "Service not running — starting it first…");
        println!();
        print_divider();
        println!();

        let log_path = filesystem::run_log_path(&cwd);
        if let Some(p) = log_path.parent() {
            fs::create_dir_all(p).ok();
        }

        let script_key = "start";
        let script = cfg
            .scripts
            .get(script_key)
            .or_else(|| cfg.scripts.values().next())
            .ok_or_else(|| HiveError::Config("No scripts defined in .hive/config.yml".into()))?;

        let cmd = match script {
            RunScript::Single(c) => c.clone(),
            RunScript::Multiple(cmds) => cmds
                .first()
                .cloned()
                .ok_or_else(|| HiveError::Config("Script has no commands".into()))?,
        };

        // Spawn service in background thread
        let cmd_clone = cmd.clone();
        let log_clone = log_path.clone();
        std::thread::spawn(move || {
            let pg = ProcessGroup::new(log_clone);
            let _ = pg.run_concurrent(vec![("app".to_string(), cmd_clone)]);
        });

        // Wait for port to open (up to 15 s)
        print_step("⟳", &format!("Waiting for port {}…", port));
        let mut ready = false;
        for _ in 0..30 {
            std::thread::sleep(Duration::from_millis(500));
            if check_port_open(port) {
                ready = true;
                break;
            }
        }
        if !ready {
            print_warn(&format!(
                "Port {} did not open in 15 s — starting tunnel anyway",
                port
            ));
        } else {
            print_success(&format!("Port {} is up", port));
        }
        println!();
        print_divider();
        println!();
    } else {
        print_success(&format!("Service already running on port {}", port));
    }

    fs::create_dir_all(tunnel_log_dir(&cwd)).ok();
    let tlog = tunnel_log_path(&cwd);
    write_tunnel_log(&tlog, "=== Tunnel started ===", false);
    write_tunnel_log(&tlog, &format!("Local URL: {}", local_url), false);
    write_tunnel_log(&tlog, &format!("Binary:    {}", bin_path.display()), false);

    print_label("  Log", &tlog.display().to_string());
    print_label("  URL", &local_url.truecolor(147, 197, 253).to_string());
    println!();
    print_divider();
    println!();

    // ── 4. launch cloudflared ────────────────────────────────────────────────
    let mut cmd = std::process::Command::new(&bin_path);
    cmd.arg("tunnel").arg("--url").arg(&local_url);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                libc_setsid();
                Ok(())
            });
        }
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| HiveError::Process(format!("Failed to start cloudflared: {}", e)))?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    // stdout thread
    let tlog_out = tlog.clone();
    let h1 = std::thread::spawn(move || {
        use std::io::BufRead;
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            write_tunnel_log(&tlog_out, &line, false);
            if let Some(url) = extract_url(&line) {
                println!();
                println!(
                    "  {} {}",
                    "🌐".to_string(),
                    "Public URL:".truecolor(255, 200, 0).bold()
                );
                println!();
                println!("     {}", url.truecolor(147, 197, 253).bold().underline());
                println!();
                write_tunnel_log(&tlog_out, &format!("Public URL: {}", url), false);
            } else {
                println!(
                    "  {}  {}",
                    "[tunnel]".truecolor(100, 100, 100),
                    line.truecolor(160, 160, 160)
                );
            }
        }
    });

    // stderr thread (cloudflared writes most output here)
    let tlog_err = tlog.clone();
    let h2 = std::thread::spawn(move || {
        use std::io::BufRead;
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            let is_err =
                line.to_lowercase().contains("error") || line.to_lowercase().contains("failed");
            write_tunnel_log(&tlog_err, &line, is_err);
            if let Some(url) = extract_url(&line) {
                println!();
                println!("  {} {}", "🌐", "Public URL:".truecolor(255, 200, 0).bold());
                println!();
                println!("     {}", url.truecolor(147, 197, 253).bold().underline());
                println!();
                write_tunnel_log(&tlog_err, &format!("Public URL: {}", url), false);
            } else if is_err {
                println!(
                    "  {}  {}",
                    "[tunnel]".truecolor(220, 80, 80),
                    line.truecolor(220, 130, 130)
                );
            } else {
                println!(
                    "  {}  {}",
                    "[tunnel]".truecolor(100, 100, 100),
                    line.truecolor(160, 160, 160)
                );
            }
        }
        write_tunnel_log(&tlog_err, "=== Tunnel stopped ===", false);
    });

    print_step("⟳", "Tunnel running… (Ctrl+C to stop)");
    println!();

    let _ = child.wait();
    let _ = h1.join();
    let _ = h2.join();

    println!();
    print_divider();
    println!();
    print_success("Tunnel closed.");
    println!();
    Ok(())
}

fn check_port_open(port: u16) -> bool {
    std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok()
}

#[cfg(unix)]
fn libc_setsid() {
    extern "C" {
        fn setsid() -> i32;
    }
    unsafe {
        setsid();
    }
}
