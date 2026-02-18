# capture

A Rust CLI to capture and retrieve output of long-running processes by name. Built for AI-assisted development workflows where agents need easy access to server logs, build output, etc.

## Usage

```bash
# Start a named capture
capture start --name backend "npm run dev"
capture start --name frontend "npm start"

# Get full output
capture logs backend

# Last N lines
capture logs backend --lines 50

# First N lines
capture logs backend --head 20

# Grep output
capture logs backend --grep "ERROR"

# Tail output in real-time
capture logs backend --follow

# Stderr only
capture logs backend --stderr

# Combine flags
capture logs backend --lines 100 --grep "panic" --stderr

# List active captures
capture list

# Stop a capture
capture stop backend

# Stop all
capture stop --all
```

## Why

When an AI agent tests code, it often needs to check server output for errors. Without this tool, the agent has no easy way to read stdout/stderr of a background process it launched. `capture` solves this by:

1. Running the process in the background
2. Storing stdout/stderr to a retrievable buffer
3. Letting any other process (or AI agent) query the output by name

## Architecture

- Processes run as managed background daemons
- Output stored in `~/.capture/<name>/` (stdout, stderr, pid)
- CLI communicates with running captures via the filesystem
- Named captures are unique per user session

## Install

### Quick install (linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/AloisH/capture-cli/main/install.sh | sh
```

Installs a prebuilt binary to `/usr/local/bin`. Override with `INSTALL_DIR=~/.local/bin`.

### From source

```bash
git clone https://github.com/AloisH/capture-cli.git
cd capture-cli
cargo build --release
# binary at target/release/capture
```

### Via cargo

```bash
cargo install capture-cli
```
