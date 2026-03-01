# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-02-28

### Added

- Initial release of CompScan
- **CLI**: Full command interface with `scan`, `observe`, `dashboard`, `web`, `report`, `act`, `status`, `config` subcommands
- **System Scanner**: Hardware inventory, process analysis, disk audit, startup items, security audit
- **Activity Observer**: Background daemon with configurable sampling interval
- **Rule Engine**: 30+ analysis rules across performance, security, productivity, and habits
- **Statistical Analysis**: Z-score anomaly detection on CPU, memory, and process count
- **Ollama Integration**: Hybrid AI with graceful degradation when Ollama is unavailable
- **Action System**: Trust-level permission model (Low/Medium/High/Critical)
- **Built-in Actions**: Cache cleanup, temp cleanup, log cleanup, disk cleanup, process management
- **TUI Dashboard**: Multi-tab interactive terminal dashboard with ratatui
- **Web Dashboard**: Responsive localhost dashboard with REST API at port 7890
- **Storage**: redb embedded database with ACID transactions
- **Encryption**: AES-256-GCM encryption with Argon2 key derivation
- **Cross-Platform**: macOS, Linux, Windows support via sysinfo
- **Security Audit**: SSH key permissions, sensitive files, firewall status checks

### Security

- Zero network egress (localhost Ollama only)
- Encrypted data at rest
- Sandboxed action execution with timeouts
- Full audit trail of all actions

[Unreleased]: https://github.com/vvk147/comp-scan/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/vvk147/comp-scan/releases/tag/v0.1.0
