# CompScan — Project Brief

## Vision
A fully local AI agent that observes, learns, and optimizes the user's digital life. Runs as a single CLI command, works across all operating systems, with zero security vulnerabilities and zero network egress.

## Core Requirements
1. Single CLI command to scan, observe, analyze, and act
2. Cross-platform support (macOS, Linux, Windows)
3. Resource efficient — minimal CPU/memory footprint
4. Hybrid AI — rule engine + Ollama for reasoning
5. Trust-level permission system for automated actions
6. Dual dashboard — TUI and web
7. Encrypted local storage
8. Zero network egress, zero telemetry

## Implementation
- Language: Rust
- Database: redb (embedded)
- TUI: ratatui + crossterm
- Web: axum with embedded HTML
- AI: Ollama REST client
- Encryption: AES-256-GCM + Argon2
