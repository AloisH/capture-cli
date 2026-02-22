---
name: capture
description: Run and monitor long-running CLI processes by name using the `capture` tool
---

# capture — background process runner

Use `capture` whenever you need to run a long-running process (dev servers, builds, watchers, test suites) and retrieve its output later. It captures stdout and stderr to disk so you can inspect logs without keeping a terminal open.

## Install

```bash
npm i -g @heloir/capture
```

## Commands

### Start a capture

```bash
capture start --name <n> -- <cmd...>
```

Runs `<cmd>` in the background and captures its output under the name `<n>`. If a capture with that name already exists, it is stopped and overridden.

### Read logs

```bash
capture logs <name>
```

| Flag | Description |
|------|-------------|
| `-l, --lines <N>` | Last N lines (tail) |
| `--head <N>` | First N lines |
| `-g, --grep <pattern>` | Filter lines by pattern |
| `-f, --follow` | Stream output in real-time |
| `--stderr` | Show stderr only |

### List active captures

```bash
capture list
```

### Stop a capture

```bash
capture stop <name>
capture stop --all
```

## When to use

- Long builds (`cargo build`, `npm run build`)
- Dev servers (`npm run dev`, `flask run`)
- Background tasks (migrations, seeds, file watchers)
- Any command whose output you want to check later

## Limitation

Interactive CLIs (prompts, TUIs) are not supported — `capture` does not allocate a PTY.
