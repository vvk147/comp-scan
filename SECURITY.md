# Security Policy

## Design Principles

CompScan is designed with security as a core principle:

1. **Zero Network Egress** — No data ever leaves your machine. The only network connection is to localhost Ollama (127.0.0.1:11434).
2. **Encrypted Storage** — All observation data can be encrypted at rest using AES-256-GCM with Argon2 key derivation.
3. **Sandboxed Actions** — All automated actions execute in subprocesses with timeouts.
4. **Trust Levels** — Four-tier permission model prevents unauthorized or dangerous actions.
5. **No Telemetry** — Zero analytics, zero tracking, zero phone-home behavior.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do NOT** open a public GitHub issue
2. Email: vkyaligar@gmail.com
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will acknowledge receipt within 48 hours and provide a timeline for resolution.

## Scope

### In Scope

- Data leakage (observation data leaving the machine)
- Encryption weaknesses in the storage layer
- Action execution bypassing permission checks
- Privilege escalation through the action system
- Injection vulnerabilities in shell commands

### Out of Scope

- Vulnerabilities in Ollama itself (report to Ollama project)
- Physical access attacks
- Social engineering

## Security Checklist for Contributors

When contributing code, verify:

- [ ] No new network calls added (except localhost Ollama)
- [ ] No data written outside the designated data directory
- [ ] Actions use appropriate risk levels
- [ ] Shell commands are properly sanitized
- [ ] Sensitive data (credentials, keys) is never logged
- [ ] New dependencies are audited (`cargo audit`)
