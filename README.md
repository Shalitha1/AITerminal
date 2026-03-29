# AI Terminal

A native desktop terminal app powered by **Google Gemini AI**. Type natural language and it instantly translates it into shell commands — no need to memorize syntax.

![AI Terminal](https://img.shields.io/badge/Built%20with-Tauri%20%2B%20Rust-orange?style=flat-square) ![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows-blue?style=flat-square) ![AI](https://img.shields.io/badge/AI-Google%20Gemini-green?style=flat-square)

---

## Features

- **Natural language to shell commands** — Type "go to downloads" and it runs `cd ~/Downloads`
- **Real-time AI streaming** — See the translated command appear token by token
- **Auto-retry on failure** — If a command fails, AI automatically suggests and runs a fix
- **Safety checks** — Dangerous commands are blocked or warned before execution
- **Command history** — Navigate previous commands with ↑ / ↓ arrow keys
- **Working directory tracking** — Always shows your current path
- **Conversation context** — AI remembers previous commands for better suggestions

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop framework | [Tauri v1](https://tauri.app) |
| Backend | Rust |
| Frontend | Plain HTML + CSS + JavaScript |
| AI Model | Google Gemini 2.0 Flash |
| HTTP Client | reqwest |
| Async Runtime | Tokio |

---

## Prerequisites

Before running, make sure you have the following installed:

- **Rust** — https://rustup.rs
- **Node.js** (for the static file server) — https://nodejs.org
- **Tauri CLI v1**
- **A Google Gemini API key** — https://aistudio.google.com/apikey (free)

---

## Installation & Setup

### 1. Clone the repository

```bash
git clone https://github.com/Shalitha1/AITerminal.git
cd AITerminal
```

### 2. Install Tauri CLI

```bash
cargo install tauri-cli --version "^1"
```

### 3. Install the static file server

```bash
npm install -g serve
```

### 4. Get a Gemini API Key

1. Go to https://aistudio.google.com/apikey
2. Click **Create API Key**
3. Copy the key (it starts with `AIzaSy...`)

---

## Running in Development

You need **two terminals** open at the same time.

### Terminal 1 — Start the frontend server

```bash
cd /path/to/AITerminal
serve -p 4173 frontend
```

### Terminal 2 — Start the Tauri app

```bash
cd /path/to/AITerminal
cargo tauri dev
```

The app window will open automatically once compilation finishes (first run may take a few minutes).

### Terminal 3 — Enter your API key

When the app opens, paste your Gemini API key into the input field and press **Enter** or click **Connect**.

---

## Building for Production

To build a standalone `.app` (macOS) or `.exe` (Windows):

```bash
cargo tauri build
```

The output will be at:
- **macOS**: `target/release/bundle/macos/AI Terminal.app`
- **Windows**: `target/release/bundle/msi/AI Terminal.msi`

Just double-click to run — no terminal needed.

---

## Project Structure

```
AITerminal/
├── src/
│   ├── main.rs        # Tauri commands, app state
│   ├── ai.rs          # Gemini API client, streaming
│   ├── shell.rs       # Shell command execution
│   └── safety.rs      # Command safety checks
├── frontend/
│   └── index.html     # UI (HTML + CSS + JS)
├── icons/
│   └── icon.png       # App icon
├── build.rs           # Tauri build script
├── Cargo.toml         # Rust dependencies
└── tauri.conf.json    # Tauri configuration
```

---

## Usage Examples

| You type | AI runs |
|----------|---------|
| `go to downloads` | `cd ~/Downloads` |
| `list all files` | `ls -la` |
| `make a folder called test` | `mkdir test` |
| `what's my ip` | `curl ifconfig.me` |
| `show disk usage` | `df -h` |
| `find all pdf files` | `find . -name "*.pdf"` |
| `pwd` | `pwd` *(passed through as-is)* |

---

## Configuration

The app is configured via `tauri.conf.json`. Key settings:

```json
{
  "build": {
    "devPath": "http://localhost:4173",
    "distDir": "frontend",
    "withGlobalTauri": true
  }
}
```

---

## Troubleshooting

**White screen on launch**
Make sure the frontend server is running on port 4173 before starting `cargo tauri dev`.

**API key not working**
Make sure you're using a Google Gemini API key from https://aistudio.google.com/apikey — not an Anthropic key.

**Commands return no output**
Check that your Gemini API key is valid and has not exceeded the free tier quota.

**Build fails with icon error**
Make sure `icons/icon.png` exists. Create one with:
```bash
mkdir -p icons && python3 -c "
from PIL import Image
img = Image.new('RGB', (512, 512), color=(137, 180, 250))
img.save('icons/icon.png')
"
```

---

## License

MIT
