use crate::error::HiveResult;
use chrono::Local;
use colored::*;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;

pub struct ProcessGroup {
    log_path: PathBuf,
}

impl ProcessGroup {
    pub fn new(log_path: PathBuf) -> Self {
        Self { log_path }
    }

    pub fn run_concurrent(&self, scripts: Vec<(String, String)>) -> HiveResult<()> {
        let log_file = Arc::new(Mutex::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_path)?,
        ));

        let should_stop = Arc::new(AtomicBool::new(false));

        // shared list of child PIDs so we can kill them all
        let children_pids: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![]));

        let colors: &[fn(&str) -> ColoredString] = &[
            |s: &str| s.truecolor(147, 197, 253),
            |s: &str| s.truecolor(196, 181, 253),
            |s: &str| s.truecolor(251, 113, 133),
            |s: &str| s.truecolor(253, 186, 116),
            |s: &str| s.truecolor(134, 239, 172),
            |s: &str| s.truecolor(103, 232, 249),
        ];

        // Ctrl+C handler — kill everything and exit clean
        {
            let stop = Arc::clone(&should_stop);
            let pids = Arc::clone(&children_pids);
            let _ = ctrlc::set_handler(move || {
                stop.store(true, Ordering::SeqCst);
                let pids = pids.lock().unwrap();
                for &pid in pids.iter() {
                    kill_pid(pid);
                }
                println!();
                println!(
                    "  {} {}",
                    "◆".truecolor(255, 160, 0).bold(),
                    "Interrupted.".truecolor(200, 160, 80)
                );
                std::process::exit(0);
            });
        }

        let mut handles = vec![];

        for (idx, (name, cmd)) in scripts.into_iter().enumerate() {
            let log_file = Arc::clone(&log_file);
            let color = colors[idx % colors.len()];
            let stop = Arc::clone(&should_stop);
            let pids = Arc::clone(&children_pids);

            let handle = thread::spawn(move || {
                let parts = shell_words(&cmd);
                if parts.is_empty() {
                    return;
                }

                let mut child: Child = match Command::new(&parts[0])
                    .args(&parts[1..])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(c) => c,
                    Err(e) => {
                        let msg = format!("[{}] Failed to start '{}': {}", name, cmd, e);
                        eprintln!(
                            "  {} {}",
                            "✗".truecolor(220, 60, 60).bold(),
                            msg.truecolor(220, 100, 100)
                        );
                        // one process failed to start — signal others to stop
                        stop.store(true, Ordering::SeqCst);
                        return;
                    }
                };

                // register PID
                if let Ok(mut p) = pids.lock() {
                    p.push(child.id());
                }

                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();

                let name_out = name.clone();
                let log1 = Arc::clone(&log_file);
                let stop1 = Arc::clone(&stop);

                let h1 = thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines().flatten() {
                        if stop1.load(Ordering::SeqCst) {
                            break;
                        }
                        let prefix = format!("[{}]", name_out);
                        println!(
                            "  {}  {}",
                            color(&prefix).bold(),
                            line.truecolor(220, 220, 220)
                        );
                        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
                        let log_line = format!("{} [{}] {}\n", ts, name_out, line);
                        if let Ok(mut f) = log1.lock() {
                            let _ = f.write_all(log_line.as_bytes());
                        }
                    }
                });

                let name_err = name.clone();
                let log2 = Arc::clone(&log_file);
                let stop2 = Arc::clone(&stop);

                let h2 = thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines().flatten() {
                        if stop2.load(Ordering::SeqCst) {
                            break;
                        }
                        let prefix = format!("[{}]", name_err);
                        println!(
                            "  {}  {}",
                            color(&prefix).bold(),
                            line.truecolor(220, 130, 130)
                        );
                        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
                        let log_line = format!("{} [{}] [stderr] {}\n", ts, name_err, line);
                        if let Ok(mut f) = log2.lock() {
                            let _ = f.write_all(log_line.as_bytes());
                        }
                    }
                });

                let status = child.wait();
                let _ = h1.join();
                let _ = h2.join();

                // if any process exits (crashed or finished), kill all others
                let crashed = match status {
                    Ok(s) if s.success() => false,
                    _ => true,
                };

                if crashed && !stop.load(Ordering::SeqCst) {
                    stop.store(true, Ordering::SeqCst);
                    let msg = format!("[{}] exited — stopping all processes", name);
                    eprintln!(
                        "  {}  {}",
                        "◆".truecolor(255, 160, 0).bold(),
                        msg.truecolor(200, 160, 80)
                    );
                    let pids = pids.lock().unwrap();
                    for &pid in pids.iter() {
                        kill_pid(pid);
                    }
                }
            });

            handles.push(handle);
        }

        for h in handles {
            let _ = h.join();
        }

        Ok(())
    }
}

fn kill_pid(pid: u32) {
    #[cfg(unix)]
    unsafe {
        libc_kill(pid as i32);
    }
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output();
    }
}

#[cfg(unix)]
fn libc_kill(pid: i32) {
    use std::os::unix::process::ExitStatusExt;
    unsafe {
        // SIGTERM first
        libc_sys_kill(pid, 15);
    }
}

#[cfg(unix)]
extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

#[cfg(unix)]
unsafe fn libc_sys_kill(pid: i32, sig: i32) {
    kill(pid, sig);
}

pub fn shell_words(s: &str) -> Vec<String> {
    let mut words = vec![];
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for c in s.chars() {
        match c {
            '"' | '\'' if !in_quote => {
                in_quote = true;
                quote_char = c;
            }
            c if in_quote && c == quote_char => {
                in_quote = false;
            }
            ' ' if !in_quote => {
                if !current.is_empty() {
                    words.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}
