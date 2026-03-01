# Contributing to CompScan

Thank you for your interest in contributing to CompScan. This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating, you agree to uphold our [Code of Conduct](CODE_OF_CONDUCT.md).

## How to Contribute

### Reporting Issues

- Use [GitHub Issues](https://github.com/vvk147/comp-scan/issues) with the appropriate template
- Include your OS, Rust version, and `compscan --version` output
- For bugs, include steps to reproduce
- For features, describe the use case

### Pull Requests

1. Fork the repository
2. Create a feature branch from `main`: `git checkout -b feat/your-feature`
3. Make your changes following the guidelines below
4. Run the checks: `cargo fmt && cargo clippy && cargo test`
5. Commit with conventional commits (see below)
6. Push and open a PR against `main`

### Branch Naming

- `feat/` — New features
- `fix/` — Bug fixes
- `docs/` — Documentation changes
- `refactor/` — Code refactoring
- `ci/` — CI/CD changes

### Commit Messages

Use this format (short subject, optional description):

```
✨ feat: <message in as few words as possible>

- description only if required
- more description if needed
```

Examples:

```
✨ feat: add disk usage trend analysis
✨ feat: initial CompScan release

- Local AI system optimization agent
fix: correct memory calculation on macOS ARM
docs: update installation instructions
refactor: extract common truncation helper
ci: add cross-platform test matrix
```

## Development Setup

### Prerequisites

- Rust 1.75+ (`rustup update stable`)
- Optional: Ollama for AI features (`ollama pull llama3.2`)

### Building

```bash
git clone https://github.com/vvk147/comp-scan.git
cd comp-scan
cargo build
cargo run -- scan
```

### Testing

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Project Structure

```
src/
├── cli/          # Command-line interface (clap)
├── scanner/      # System scanning modules
├── observer/     # Activity tracking daemon
├── analyzer/     # Rule engine + statistics
├── ai/           # Ollama integration
├── actions/      # Action execution + permissions
├── storage/      # Database + encryption
├── ui/           # TUI + Web dashboards
└── daemon/       # Background scheduler
```

## Contribution Areas

### Adding Analysis Rules

Rules live in `src/analyzer/rules.rs`. Each rule:
1. Inspects system state (snapshot or activity records)
2. Produces an `Insight` with category, severity, title, description, suggestion
3. Optionally links to an action via `action_id`

```rust
// Example rule
if snapshot.process_count > 500 {
    insights.push(Insight {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        category: InsightCategory::Performance,
        severity: InsightSeverity::Warning,
        title: "Very high process count".into(),
        description: "...".into(),
        suggestion: "...".into(),
        action_id: None,
        source: InsightSource::RuleEngine,
    });
}
```

### Adding Actions

Actions live in `src/actions/`. Each action has:
- `id` — Unique identifier
- `risk_level` — Low, Medium, High, or Critical
- `command` — ShellCommand, KillProcess, DeleteFiles, DisableStartupItem, or Custom
- `reversible` — Whether the action can be undone
- `estimated_impact` — Human-readable impact description

### Platform-Specific Code

Use `#[cfg(target_os = "...")]` for platform-specific implementations:

```rust
#[cfg(target_os = "macos")]
fn platform_specific() { /* macOS impl */ }

#[cfg(target_os = "linux")]
fn platform_specific() { /* Linux impl */ }

#[cfg(target_os = "windows")]
fn platform_specific() { /* Windows impl */ }
```

## AI-Assisted Development

This project includes rules and context files for AI-assisted development:

- `.cursor/rules/` — Cursor IDE rules for consistent AI suggestions
- `AGENTS.md` — Guidelines for AI agents working on this codebase
- `memory-bank/` — Project context for AI assistants

When using AI tools, stay within these boundaries:
- Do not modify security-critical code without human review
- Do not add network calls beyond localhost Ollama
- Do not store or transmit user data
- Run `cargo clippy` before committing AI-generated code

## Release Process

We use [Semantic Versioning](https://semver.org/):

- **PATCH** (0.1.x): Bug fixes, minor improvements
- **MINOR** (0.x.0): New features, new rules, new actions
- **MAJOR** (x.0.0): Breaking changes to CLI or config format

See [CHANGELOG.md](CHANGELOG.md) for release history.
