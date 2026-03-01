# Technical Context

## Stack
- Rust 1.75+ (2021 edition)
- clap v4 (CLI)
- sysinfo 0.32 (cross-platform system info)
- redb v2 (embedded database)
- ratatui 0.29 + crossterm 0.28 (TUI)
- axum 0.8 (web)
- reqwest 0.12 (Ollama HTTP client)
- tokio (async runtime)
- serde + serde_json (serialization)
- aes-gcm + argon2 (encryption)
- tracing (logging)

## Key APIs
- sysinfo: Process.name() returns OsStr, cmd() returns &[OsString]
- redb: Need ReadableTableMetadata trait for len()
- ratatui: Frame, Layout, Block, Table patterns
- axum: Router + State<Arc<AppState>> pattern

## Build
```bash
cargo build --release    # LTO enabled, stripped binary
cargo check              # Fast type checking
cargo clippy             # Linting
cargo fmt                # Formatting
```
