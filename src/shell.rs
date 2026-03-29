use std::process::Command;
use std::env;

pub struct RunResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

pub fn run(command: &str) -> RunResult {
    if command.starts_with("cd") {
        let path = command[2..].trim().trim_matches('"').trim_matches('\'');
        let expanded = expand_tilde(if path.is_empty() { "~" } else { path });
        return match env::set_current_dir(&expanded) {
            Ok(_) => RunResult {
                stdout: format!("📁 {}", expanded),
                stderr: String::new(),
                success: true,
            },
            Err(e) => RunResult {
                stdout: String::new(),
                stderr: format!("cd: {}", e),
                success: false,
            },
        };
    }

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", command]).output()
    } else {
        Command::new("sh").args(["-c", command]).output()
    };

    match output {
        Ok(out) => RunResult {
            stdout: String::from_utf8_lossy(&out.stdout).to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
            success: out.status.success(),
        },
        Err(e) => RunResult {
            stdout: String::new(),
            stderr: format!("Exec error: {}", e),
            success: false,
        },
    }
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") || path == "~" {
        let home = if cfg!(target_os = "windows") {
            env::var("USERPROFILE").unwrap_or_default()
        } else {
            env::var("HOME").unwrap_or_default()
        };
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    }
}