# CompScan

**Your computer's personal health doctor. Fully local. Fully private. Fully free.**

```
   ___                      ____
  / __\___  _ __ ___  _ __ / ___|  ___ __ _ _ __
 / /  / _ \| '_ ` _ \| '_ \___ \ / __/ _` | '_ \
/ /__| (_) | | | | | | |_) |__) | (_| (_| | | | |
\____/\___/|_| |_| |_| .__/____/ \___\__,_|_| |_|
                      |_|
```

CompScan scans your entire computer, watches how you work, finds what's slowing you down, spots security risks, nudges you toward healthier habits — and fixes issues with one click. All in a single terminal command. **Nothing ever leaves your machine.**

---

## Install (One Command)

```bash
curl -fsSL https://raw.githubusercontent.com/vvk147/comp-scan/main/install.sh | bash
```

Or manually: `git clone https://github.com/vvk147/comp-scan.git && cd comp-scan && cargo install --path .`

---

## Get Started in 60 Seconds

```bash
# 1. Health check — see what's going on right now
compscan scan

# 2. Get recommendations — AI-powered insights
compscan report

# 3. Fix something — one-click actions
compscan act cleanup-caches
```

That's it. Three commands. You just diagnosed your system, got actionable advice, and freed up gigabytes of space.

---

## What Does It Actually Do?

| You Ask | CompScan Answers |
|---------|-----------------|
| "Why is my laptop slow?" | Shows you which processes eat your CPU/RAM, detects memory leaks, finds swap thrashing |
| "Where did my disk space go?" | Finds caches, temp files, old logs — typically 2-15 GB recoverable |
| "Am I secure?" | Checks SSH keys, credentials files, firewall, permissions — gives you a score out of 100 |
| "Am I burning out?" | Detects late-night usage, marathon sessions without breaks, excessive context switching |
| "Did I forget to commit?" | Monitors your git repos for uncommitted changes |

---

## The Daily Workflow

| When | Command | Time | What You Get |
|------|---------|------|-------------|
| Morning | `compscan status` | 5 sec | Quick system pulse — anything I should know? |
| Work hours | `compscan observe` | Set & forget | Background tracking — learns your patterns silently |
| End of day | `compscan report` | 10 sec | Full insights: performance, security, habits, coding |
| Weekly | `compscan scan --full` | 30 sec | Deep scan + cleanup = fresh machine every week |
| Anytime | `compscan dashboard` | Live | Interactive terminal dashboard with real-time data |
| Anytime | `compscan web` | Live | Browser dashboard at localhost:7890 |

---

## Why Should You Use This?

### 1. It finds money under the couch cushions
Most machines have 2-15 GB of reclaimable disk space in caches, temp files, and old logs. CompScan finds them in seconds and cleans them safely.

### 2. It makes your machine measurably faster
By identifying resource hogs, unnecessary startup items, memory leaks, and swap thrashing — and letting you fix them with one click.

### 3. It catches security holes you didn't know existed
SSH keys with wrong permissions, exposed credentials, disabled firewalls — most people have 2-3 issues. CompScan finds them and tells you exactly how to fix each one.

### 4. It nudges you toward healthier habits
Working at 1 AM? Four-hour marathon session? Constant app switching? CompScan notices and gently suggests breaks, focus time, and winding down.

### 5. It's a privacy fortress
Zero network egress. No account. No cloud. No telemetry. No tracking. Everything stays on your machine, encrypted. You own your data, period.

### 6. It gets smarter over time
The longer the observer runs, the better CompScan understands your baselines. It catches anomalies earlier, patterns more accurately, and gives personalized insights.

---

## Safety: Nothing Happens Without Your Permission

Every action has a **trust level**:

| Risk | Behavior | Examples |
|------|----------|---------|
| **Low** | Asks once, safe to approve | Clean temp files, clear caches |
| **Medium** | Explains impact, confirms | Disable a startup item, full disk cleanup |
| **High** | Requires typing "yes" | Kill a process, delete large files |
| **Critical** | **Blocked** — manual only | CompScan will never touch system files |

Full audit trail of every action executed: what, when, who approved, what happened.

---

## Optional: Add AI Superpowers

CompScan works great out of the box with its built-in rule engine (30+ rules) and statistical analyzer. Want deeper reasoning? Add Ollama (free, local, private):

```bash
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull llama3.2
# Done — CompScan auto-detects it, no configuration needed
```

---

## All Commands

```bash
compscan scan              # Full system health check
compscan scan --full       # Deep scan with large file detection
compscan observe           # Start background observer (Ctrl+C to stop)
compscan report            # Generate insights report
compscan report --json     # Machine-readable JSON output
compscan dashboard         # Interactive terminal dashboard
compscan web               # Web dashboard at localhost:7890
compscan act <action-id>   # Execute a suggested action
compscan status            # Quick system summary
compscan config            # View configuration
compscan --help            # Full help
```

### Built-in Actions

```bash
compscan act cleanup-caches    # Clean old cached files (Low risk)
compscan act cleanup-temp      # Remove temp files >7 days old (Low risk)
compscan act cleanup-logs      # Remove log files >30 days old (Low risk)
compscan act cleanup-disk      # Full cleanup: npm, pip, cargo caches (Medium risk)
compscan act kill-top-memory   # Kill highest memory process (High risk)
compscan act optimize-startup  # Review startup items (Medium risk)
```

---

## How It Works (For the Curious)

```
You run compscan
        |
        v
    [Scanner] ---- hardware, processes, disk, startup, security
        |
        v
    [Observer] --- background sampling every 30s
        |
        v
    [Storage] ---- encrypted local database (redb)
        |
        v
    [Analyzer] --- 30+ rules + z-score anomaly detection
        |
        v
    [AI Layer] --- Ollama for complex reasoning (optional)
        |
        v
    [Insights] --- ranked by severity, each with a suggested fix
        |
        v
    [Actions] ---- trust-level gated, one-click execution
```

Built in Rust. 3 MB binary. Cross-platform (macOS, Linux, Windows). Single database file. Zero dependencies at runtime.

---

## FAQ

**Will it slow my computer down?** No. Built in Rust, the observer takes <200ms every 30 seconds. The release binary is 3 MB.

**Does it phone home?** Never. Zero network egress. Only connection is to local Ollama (optional, localhost only).

**Can it break anything?** No. Critical actions are blocked. High-risk actions require explicit confirmation. Low-risk actions only touch temp files and caches.

**Where's my data?** `~/Library/Application Support/compscan/` (macOS), `~/.local/share/compscan/` (Linux), `%APPDATA%/compscan/` (Windows). Delete the folder anytime to start fresh.

**How do I update?** `cd ~/.compscan && git pull && cargo install --path .`

**How do I uninstall?** `cargo uninstall compscan && rm -rf ~/.compscan`

---

## Full Documentation

- **[User Guide](docs/USER_GUIDE.md)** — Detailed walkthrough with real scenarios
- **[Contributing](CONTRIBUTING.md)** — How to add rules, actions, and features
- **[Security](SECURITY.md)** — Security model and vulnerability reporting
- **[Changelog](CHANGELOG.md)** — Release history

## License

MIT — free forever, for everyone. See [LICENSE](LICENSE).
