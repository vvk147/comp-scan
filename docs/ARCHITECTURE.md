# CompScan Architecture

This document describes the high-level architecture of CompScan: components, data flow, and where to find details in the repo.

---

## Architecture Overview

```mermaid
flowchart TB
    subgraph CLI ["CLI Entry (clap)"]
        Scan["compscan scan"]
        Observe["compscan observe"]
        Dashboard["compscan dashboard"]
        Act["compscan act"]
        Report["compscan report"]
    end

    subgraph Core ["Core Engine"]
        Scanner["System Scanner"]
        Observer["Activity Observer"]
        Analyzer["Pattern Analyzer"]
        ActionEngine["Action Engine"]
    end

    subgraph AI ["Hybrid AI Layer"]
        Rules["Rule Engine (native)"]
        Ollama["Ollama Client (localhost)"]
        Prompts["Prompt Templates"]
    end

    subgraph Storage ["Data Layer (redb)"]
        ActivityDB["Activity Store"]
        InsightsDB["Insights Store"]
        ConfigDB["Config & Trust Levels"]
        PatternsDB["Learned Patterns"]
    end

    subgraph UI ["Presentation"]
        TUI["Terminal Dashboard (ratatui)"]
        Web["Web Dashboard (axum)"]
        Notif["System Notifications"]
    end

    CLI --> Core
    Core --> AI
    Core --> Storage
    AI --> Storage
    Core --> UI
    Storage --> UI
```

- **CLI** (`src/cli/`, `src/main.rs`): Single entry point; subcommands dispatch to scanner, observer, analyzer, actions, TUI, or web.
- **Core**: Scanner (system/process/disk/startup/security), Observer (activity/coding/habits), Analyzer (rules + statistics), Action engine (executor + trust gate).
- **AI**: Rule engine (native) + Ollama client for optional LLM reasoning.
- **Storage**: redb database; models in `storage/models.rs`; optional encryption in `storage/encryption.rs`.
- **UI**: TUI (ratatui), Web (axum + embedded SPA), and cross-platform notifications.

---

## Data Flow

```mermaid
flowchart LR
    subgraph Collect ["1. Collect"]
        Sys["System Metrics"]
        Proc["Processes"]
        FS["File System"]
        App["App Usage"]
    end

    subgraph Store ["2. Store"]
        DB["redb"]
    end

    subgraph Analyze ["3. Analyze"]
        RuleEng["Rule Engine"]
        Stats["Statistics"]
        LLM["Ollama LLM (optional)"]
    end

    subgraph Present ["4. Present"]
        Insights["Ranked Insights"]
        Actions["Suggested Actions"]
    end

    subgraph Execute ["5. Execute"]
        Trust["Trust Gate"]
        Runner["Action Runner"]
    end

    Collect --> Store --> Analyze --> Present --> Execute
    Execute -->|"feedback"| Store
```

1. **Collect**: `compscan scan` and `compscan observe` populate system snapshots and activity records.
2. **Store**: All data written to redb (see `storage/db.rs`, `storage/models.rs`).
3. **Analyze**: `compscan report` runs rules and statistics over stored data to produce insights; Ollama can augment when wired in.
4. **Present**: Insights and suggested actions shown in TUI, web, or report output.
5. **Execute**: `compscan act <id>` runs actions through the trust-level gate and executor; results logged.

---

## Trust-Level Permission System

```mermaid
flowchart TD
    Action["Proposed Action"] --> Classify{"Risk Level?"}
    Classify -->|"Low Risk"| AutoApprove["Auto-Execute"]
    Classify -->|"Medium Risk"| Notify["Notify + 1-Click Approve"]
    Classify -->|"High Risk"| Confirm["Explicit Confirmation Required"]
    Classify -->|"Critical"| Block["Block + Explain Why"]
    AutoApprove --> Log["Log to Audit Trail"]
    Notify --> Log
    Confirm --> Log
    Block --> Log
```

- **Low risk (auto):** e.g. cache/temp cleanup.
- **Medium risk (notify):** e.g. startup changes, file reorganization.
- **High risk (confirm):** e.g. large file deletion, config changes.
- **Critical (block):** system files, security settings, credentials.

Implementation: `actions/permissions.rs`, `actions/executor.rs`.

---

## Project Structure (source)

| Path | Purpose |
|------|---------|
| `src/main.rs` | Entry point, CLI dispatch, first-run UX |
| `src/cli/mod.rs` | Subcommands: scan, observe, dashboard, web, report, act, status, config |
| `src/scanner/` | System snapshot, processes, filesystem, startup, security |
| `src/observer/` | Activity sampling, coding checks, habits/patterns |
| `src/analyzer/` | Rules, statistics, insight generation and report |
| `src/ai/` | Ollama client, prompts, hybrid reasoning |
| `src/actions/` | Registry, executor, permissions, cleanup, optimization, workflow |
| `src/storage/` | redb DB, models, encryption (optional) |
| `src/ui/tui/` | Ratatui dashboard (Overview, Insights, Actions, History) |
| `src/ui/web/` | Axum server, REST API, embedded HTML/JS |
| `src/ui/notifications.rs` | macOS/Linux/Windows system notifications |
| `src/daemon/` | Status display, scheduler stubs |

---

## Where to Read More

- **User-facing:** [USER_GUIDE.md](USER_GUIDE.md) — install, workflow, benefits.
- **Contributors:** [CONTRIBUTING.md](../CONTRIBUTING.md) — how to contribute.
- **Plan vs implementation:** [PLAN_AUDIT.md](PLAN_AUDIT.md) — gaps and status.
- **Docs index:** [README.md](README.md) (this folder) — list of all documentation.
- **AI/agents:** [AGENTS.md](../AGENTS.md) — for AI contributors.
- **Security:** [SECURITY.md](../SECURITY.md) — reporting and policy.
- **Memory bank:** `memory-bank/` — projectbrief, techContext, systemPatterns, progress, etc., for context across sessions.
