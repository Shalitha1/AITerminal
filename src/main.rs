mod ai;
mod shell;
mod safety;

use tokio::sync::Mutex;
use tauri::State;
use serde::Serialize;

struct AppState {
    ai: Mutex<Option<ai::AiClient>>,
    cwd: Mutex<String>,
}

#[derive(Serialize)]
struct CommandResult {
    command: String,
    stdout: String,
    stderr: String,
    success: bool,
    cwd: String,
    blocked: bool,
    warn: Option<String>,
}

// Called from frontend: set the API key
#[tauri::command]
async fn set_api_key(key: String, state: State<'_, AppState>) -> Result<String, String> {
    let client = ai::AiClient::new(key);
    *state.ai.lock().await = Some(client);
    Ok("API key set successfully".to_string())
}

// Called from frontend: run a natural language command
#[tauri::command]
async fn run_input(
    input: String,
    state: State<'_, AppState>,
    window: tauri::Window,
) -> Result<CommandResult, String> {
    let cwd = state.cwd.lock().await.clone();

    // Translate with AI (stream tokens to frontend in real-time)
    let command = {
        let mut ai_guard = state.ai.lock().await;
        let client = ai_guard.as_mut().ok_or("API key not set")?;

        let window_clone = window.clone();
        client.translate(&input, move |token| {
            let _ = window_clone.emit("stream-token", token);
        }).await?
    };

    // Safety check
    let (blocked, warn) = match safety::check(&command) {
        safety::SafetyResult::Blocked(msg) => (true, Some(msg)),
        safety::SafetyResult::Warn(msg) => (false, Some(msg)),
        safety::SafetyResult::Safe => (false, None),
    };

    if blocked {
        return Ok(CommandResult {
            command,
            stdout: String::new(),
            stderr: String::new(),
            success: false,
            cwd,
            blocked: true,
            warn,
        });
    }

    // Run the command
    let result = shell::run(&command);

    // Update cwd
    if let Ok(new_cwd) = std::env::current_dir() {
        *state.cwd.lock().await = new_cwd.display().to_string();
    }

    let new_cwd = state.cwd.lock().await.clone();

    // Auto-retry on failure
    if !result.success && !result.stderr.is_empty() {
        let fixed = {
            let mut ai_guard = state.ai.lock().await;
            let client = ai_guard.as_mut().ok_or("API key not set")?;
            let window_clone = window.clone();
            client.fix_command(&command, &result.stderr, move |token| {
                let _ = window_clone.emit("stream-fix-token", token);
            }).await?
        };

        let retry = shell::run(&fixed);
        let combined_stdout = format!("{}\n---retry: {}\n{}", result.stderr, fixed, retry.stdout);

        return Ok(CommandResult {
            command: format!("{} → fixed: {}", command, fixed),
            stdout: combined_stdout,
            stderr: retry.stderr,
            success: retry.success,
            cwd: new_cwd,
            blocked: false,
            warn,
        });
    }

    Ok(CommandResult {
        command,
        stdout: result.stdout,
        stderr: result.stderr,
        success: result.success,
        cwd: new_cwd,
        blocked: false,
        warn,
    })
}

#[tauri::command]
async fn get_cwd(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.cwd.lock().await.clone())
}

fn main() {
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "/".to_string());

    tauri::Builder::default()
        .manage(AppState {
            ai: Mutex::new(None),
            cwd: Mutex::new(cwd),
        })
        .invoke_handler(tauri::generate_handler![
            set_api_key,
            run_input,
            get_cwd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}