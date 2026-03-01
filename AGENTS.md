# AI Agent Guidelines for CompScan

This document provides guidelines for AI agents (Cursor, Copilot, Codex, etc.) working on this codebase.

## Project Context

CompScan is a **fully local** AI agent that monitors system activity and provides optimization suggestions. It is written in Rust and runs entirely on the user's machine.

## Critical Constraints

### Security Rules (NEVER violate)

1. **No network egress** — Never add code that sends data to external servers. The only allowed network call is to localhost Ollama (127.0.0.1:11434).
2. **No telemetry** — Never add analytics, tracking, or phone-home behavior.
3. **No credential storage** — Never store user passwords, API keys, or tokens in the database.
4. **No unsafe code** — Avoid `unsafe` blocks unless absolutely necessary and thoroughly justified.
5. **Sanitize shell commands** — Never pass unsanitized user input to shell commands.

### Architecture Rules

1. **Cross-platform** — All new features must work on macOS, Linux, and Windows. Use `#[cfg(target_os = "...")]` for platform-specific code.
2. **Resource efficient** — CompScan must be lightweight. Avoid spawning threads unnecessarily, minimize allocations, use streaming where possible.
3. **Graceful degradation** — Features should work without Ollama. Never make Ollama a hard dependency.
4. **Error handling** — Use `anyhow::Result` for application errors, `thiserror` for library errors. Never `unwrap()` in production code paths.

### Code Style

1. Run `cargo fmt` before committing
2. Run `cargo clippy` and fix all warnings
3. Use `tracing` for logging, not `println!` (except in CLI output)
4. Follow existing module patterns (see `scanner/` as reference)
5. Write doc comments for public functions
6. Use conventional commit messages

## Module Guide

| Module | Purpose | Key Files |
|--------|---------|-----------|
| `cli/` | Command parsing | `mod.rs` (Clap derive) |
| `scanner/` | System analysis | `system.rs`, `processes.rs`, `filesystem.rs`, `security.rs` |
| `observer/` | Activity tracking | `activity.rs`, `habits.rs`, `coding.rs` |
| `analyzer/` | Insight generation | `rules.rs` (add rules here), `statistics.rs` |
| `ai/` | Ollama integration | `ollama.rs`, `prompts.rs`, `reasoning.rs` |
| `actions/` | Automated actions | `executor.rs`, `permissions.rs`, `cleanup.rs` |
| `storage/` | Database layer | `db.rs` (redb), `models.rs` (data types) |
| `ui/tui/` | Terminal dashboard | `dashboard.rs`, `insights.rs`, `actions.rs` |
| `ui/web/` | Web dashboard | `server.rs` (Axum), `templates.rs` (HTML) |

## Common Tasks

### Adding a New Rule

1. Edit `src/analyzer/rules.rs`
2. Add your rule to `evaluate_snapshot()` or `evaluate_activities()`
3. Set appropriate `InsightCategory`, `InsightSeverity`
4. Optionally link to an action via `action_id`

### Adding a New Action

1. Create or edit a file in `src/actions/`
2. Define the `Action` struct with proper `RiskLevel`
3. Register it in `src/actions/mod.rs` → `get_builtin_action()`

### Adding a New CLI Command

1. Add variant to `Commands` enum in `src/cli/mod.rs`
2. Handle it in `src/main.rs` match block

### Adding Platform-Specific Code

```rust
#[cfg(target_os = "macos")]
fn do_thing() { /* macOS */ }

#[cfg(target_os = "linux")]
fn do_thing() { /* Linux */ }

#[cfg(target_os = "windows")]
fn do_thing() { /* Windows */ }
```

## Data Model

All data types are in `src/storage/models.rs`. Key types:

- `SystemSnapshot` — Point-in-time system state
- `ActivityRecord` — Periodic activity sample
- `Insight` — Generated analysis finding
- `Action` — Executable improvement action
- `ActionLog` — Audit trail entry

## Testing

- Unit tests go in the same file as the code (`#[cfg(test)]` module)
- Integration tests go in `tests/`
- Always test cross-platform behavior
