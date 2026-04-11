use anyhow::Result;
use std::path::PathBuf;

fn pid_file(port: u16) -> PathBuf {
    snact_core::data_dir().join(format!("chrome-{port}.pid"))
}

pub fn run_launch(
    port: u16,
    headless: bool,
    background: bool,
    profile: Option<&str>,
    fmt: &str,
) -> Result<()> {
    // Check if already running
    if let Some(pid) = read_pid(port) {
        if is_process_alive(pid) {
            if fmt == "json" {
                println!(
                    "{}",
                    serde_json::json!({"status": "already_running", "port": port, "pid": pid})
                );
            } else {
                eprintln!("Chrome already running on port {} (pid {})", port, pid);
            }
            return Ok(());
        }
        // Stale pid file
        std::fs::remove_file(pid_file(port)).ok();
    }

    // Persistent profile directory — keeps cookies/state between sessions
    let profile_name = profile.unwrap_or("default");
    let profile_dir = snact_core::data_dir().join("profiles").join(profile_name);
    let browser = snact_cdp::ManagedBrowser::launch(port, headless, &profile_dir)?;
    let pid = browser.pid();

    // Save PID to file
    std::fs::write(pid_file(port), pid.to_string())?;

    if fmt == "json" {
        println!(
            "{}",
            serde_json::json!({"status": "launched", "port": port, "pid": pid, "background": background})
        );
    } else {
        println!("Chrome launched on port {} (pid {})", port, pid);
    }

    if background {
        // Detach — let Chrome run independently
        std::mem::forget(browser);
    } else {
        println!("Press Ctrl+C to stop");
        std::thread::park();
    }

    Ok(())
}

pub fn run_stop(port: u16, fmt: &str) -> Result<()> {
    if let Some(pid) = read_pid(port) {
        if is_process_alive(pid) {
            kill_process(pid);
            std::fs::remove_file(pid_file(port)).ok();
            if fmt == "json" {
                println!(
                    "{}",
                    serde_json::json!({"status": "stopped", "port": port, "pid": pid})
                );
            } else {
                println!("Chrome stopped (pid {})", pid);
            }
        } else {
            std::fs::remove_file(pid_file(port)).ok();
            if fmt == "json" {
                println!(
                    "{}",
                    serde_json::json!({"status": "not_running", "port": port})
                );
            } else {
                println!("Chrome not running on port {}", port);
            }
        }
    } else if fmt == "json" {
        println!(
            "{}",
            serde_json::json!({"status": "not_running", "port": port})
        );
    } else {
        println!("Chrome not running on port {}", port);
    }
    Ok(())
}

pub fn run_status(port: u16, fmt: &str) -> Result<()> {
    let running = read_pid(port).is_some_and(is_process_alive);
    let pid = read_pid(port);

    if fmt == "json" {
        println!(
            "{}",
            serde_json::json!({
                "port": port,
                "running": running,
                "pid": pid,
            })
        );
    } else if running {
        println!("Chrome running on port {} (pid {})", port, pid.unwrap());
    } else {
        println!("Chrome not running on port {}", port);
    }
    Ok(())
}

fn read_pid(port: u16) -> Option<u32> {
    std::fs::read_to_string(pid_file(port))
        .ok()?
        .trim()
        .parse()
        .ok()
}

fn is_process_alive(pid: u32) -> bool {
    std::process::Command::new("kill")
        .args(["-0", &pid.to_string()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn kill_process(pid: u32) {
    std::process::Command::new("kill")
        .arg(pid.to_string())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .ok();
}
