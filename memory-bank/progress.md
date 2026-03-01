# Progress

## Completed
- [x] Project foundation (Cargo.toml, CLI, storage)
- [x] System scanner (hardware, processes, filesystem, startup, security)
- [x] Activity observer daemon
- [x] Rule engine (30+ rules)
- [x] Statistical analysis (z-score anomaly detection)
- [x] Ollama integration (hybrid AI with graceful degradation)
- [x] Action system (trust-level permissions, built-in actions)
- [x] TUI dashboard (4 tabs: Overview, Insights, Actions, History)
- [x] Web dashboard (REST API, embedded SPA)
- [x] GitHub repo setup (README, CONTRIBUTING, LICENSE, CI/CD, templates)
- [x] AI contributor rules (AGENTS.md, .cursorrules, .cursor/rules/)
- [x] Memory bank documentation

## In Progress
- [ ] Wire Ollama (HybridAI) into report pipeline
- [ ] Use encryption layer for DB read/write when enabled

## Remaining
- [ ] Unit tests
- [ ] Integration tests
- [ ] TUI Settings tab (plan specified 5 tabs)
- [ ] WebSocket for web dashboard real-time updates
- [ ] Optional: GPU in system snapshot, network config snapshot
- [ ] Optional: Active window tracking (platform APIs)
- [ ] Optional: `notify` crate for file watcher; wire system notifications on new insights
- [ ] Plugin system
- [ ] Trend visualization
- [ ] Browser extension integration
- [ ] Installer scripts (install.sh exists)
- [ ] crates.io publish

## Plan vs implementation
- See **docs/PLAN_AUDIT.md** for full audit (missing/partial items, doc checklist, todo status).
