pub enum SafetyResult {
    Blocked(String),
    Warn(String),
    Safe,
}

const BLOCKED: &[&str] = &[
    "rm -rf /", "rm -rf /*", "mkfs",
    "dd if=/dev/zero", ":(){:|:&};:",
    "format c:", "del /f /s /q c:\\",
];

const WARN_PATTERNS: &[&str] = &[
    "rm -rf", "sudo rm", "DROP TABLE",
    "DROP DATABASE", "del /f", "rd /s",
];

pub fn check(command: &str) -> SafetyResult {
    let lower = command.to_lowercase();
    for b in BLOCKED {
        if lower.contains(&b.to_lowercase()) {
            return SafetyResult::Blocked(format!("'{}' is blocked for safety", b));
        }
    }
    for w in WARN_PATTERNS {
        if lower.contains(&w.to_lowercase()) {
            return SafetyResult::Warn(format!("'{}' can be destructive", w));
        }
    }
    SafetyResult::Safe
}