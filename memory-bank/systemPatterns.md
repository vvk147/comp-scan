# System Patterns

## Architecture
- Modular: each concern in its own module (scanner, observer, analyzer, ai, actions, storage, ui)
- Shared data models in storage/models.rs
- Database passed by reference through all layers
- Async where needed (observer, web, AI), sync where sufficient (scanner, rules)

## Data Flow
1. Scanner/Observer collects raw data
2. Data persisted to redb
3. Analyzer (rules + stats) generates Insights
4. AI layer (Ollama) adds deep reasoning when available
5. Insights presented via TUI/web
6. Actions offered with trust-level gating
7. Execution logged to audit trail

## Trust-Level Pattern
```
Low risk     → auto-approve (cache cleanup)
Medium risk  → notify + 1-click approve
High risk    → explicit "yes" confirmation
Critical     → blocked, manual only
```

## Cross-Platform Pattern
```rust
#[cfg(target_os = "macos")]
fn platform_fn() { /* macOS impl */ }

#[cfg(target_os = "linux")]
fn platform_fn() { /* Linux impl */ }

#[cfg(target_os = "windows")]
fn platform_fn() { /* Windows impl */ }
```
