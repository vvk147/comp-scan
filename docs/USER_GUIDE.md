# CompScan User Guide

_Your personal system health doctor — always watching, always improving, completely private._

---

## Table of Contents

1. [How Do I Get This Running?](#1-how-do-i-get-this-running)
2. [What Happens When I Run It?](#2-what-happens-when-i-run-it)
3. [The Daily Workflow](#3-the-daily-workflow)
4. [What Benefits Do I Actually Get?](#4-what-benefits-do-i-actually-get)
5. [Why Should I Use This Every Day?](#5-why-should-i-use-this-every-day)
6. [Real Scenarios](#6-real-scenarios-where-compscan-saves-you)
7. [Command Reference](#7-command-reference)
8. [FAQ](#8-faq)

---

## 1. How Do I Get This Running?

### The 30-Second Way (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/vvk147/comp-scan/main/install.sh | bash
```

That's it. One command. It installs Rust (if needed), builds CompScan, and you're ready.

### The Manual Way

```bash
# Step 1: Install Rust (skip if you already have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Step 2: Clone and build
git clone https://github.com/vvk147/comp-scan.git
cd comp-scan
cargo install --path .

# Step 3: Run your first scan
compscan scan
```

### What About Ollama (AI Features)?

CompScan works perfectly **without** Ollama. The built-in rule engine and statistical analyzer handle 90% of insights. But if you want deeper, AI-powered reasoning:

```bash
# Install Ollama (free, local, private)
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull llama3.2
# CompScan auto-detects it — no configuration needed
```

---

## 2. What Happens When I Run It?

### Your First Scan

```bash
compscan scan
```

In about 5 seconds, CompScan does a full checkup of your machine:

```
  [1/5] Scanning hardware & system info...    → Who is your machine? How healthy is it?
  [2/5] Analyzing running processes...        → What's eating your CPU and memory right now?
  [3/5] Auditing disk usage...                → Where did all your storage go?
  [4/5] Checking startup items...             → What's slowing down your boot?
  [5/5] Running security audit...             → Are there any doors left open?
```

You get a **Security Score** (0-100), a list of **resource hogs**, **reclaimable disk space**, and **startup items** you probably don't need.

### Get Actionable Insights

```bash
compscan report
```

This analyzes everything CompScan has collected and tells you:
- What's **critical** (fix now)
- What's a **warning** (fix soon)
- What's a **suggestion** (nice to do)
- What's **info** (good to know)

Each insight comes with a concrete suggestion, and many have one-click fixes.

### Fix Things Instantly

```bash
compscan act cleanup-caches    # Free up GB of disk space
compscan act cleanup-temp      # Clean temporary files
compscan act cleanup-logs      # Remove old log files
compscan act cleanup-disk      # Full cleanup (npm, pip, cargo caches)
```

Before executing anything, CompScan tells you exactly what it will do, how risky it is, and asks for your permission. **Nothing happens without your approval.**

---

## 3. The Daily Workflow

Think of CompScan like a fitness tracker for your computer. Here's how power users use it:

### Morning: Quick Health Check (10 seconds)

```bash
compscan status
```
Glance at your system state. Is memory high? Any new insights since yesterday?

### During Work: Background Observer (set and forget)

```bash
compscan observe
```
Runs silently in the background. Samples your system every 30 seconds. Detects:
- Are you in a focus session or bouncing between apps?
- Is a process leaking memory over time?
- Have you been working for hours without a break?
- Do you have uncommitted code that could be lost?

Leave it running in a terminal tab. It uses virtually zero resources.

### End of Day: Insights Report (30 seconds)

```bash
compscan report
```
After a full day of observation, the report is rich:
- "You had sustained high CPU from 2-4pm — Chrome had 40 tabs open"
- "Memory has been trending upward all day — possible leak in Docker"
- "You worked 4 hours straight without a break"
- "3 git repos have uncommitted changes"

### Weekly: Deep Scan + Cleanup (2 minutes)

```bash
compscan scan --full           # Deep filesystem analysis
compscan act cleanup-caches    # Reclaim space
compscan act cleanup-logs      # Clean old logs
```

### Visual Dashboard (Anytime)

```bash
compscan dashboard             # Terminal UI — keyboard-driven, no mouse needed
# OR
compscan web                   # Open localhost:7890 in your browser
```

Both show real-time system metrics, all insights, available actions, and execution history.

---

## 4. What Benefits Do I Actually Get?

### Benefit 1: Reclaim Disk Space (Typically 2-15 GB)

Most developers have gigabytes of accumulated caches, temp files, old logs, and build artifacts. CompScan finds them all and cleans them safely.

```
Before:  45 GB free
After:   57 GB free  (+12 GB recovered)
```

### Benefit 2: Faster Machine

CompScan identifies:
- Processes consuming CPU/memory that you don't need
- Startup items slowing your boot by 10-30 seconds
- Memory leaks that degrade performance over days
- Swap thrashing that makes everything feel sluggish

Acting on these findings makes your machine noticeably faster.

### Benefit 3: Better Security Posture

Most people have at least 2-3 security issues they don't know about:
- SSH keys with wrong permissions (anyone can read them)
- `.env` files readable by other users
- Firewall disabled
- Docker credentials exposed

CompScan catches these and tells you exactly how to fix each one.

### Benefit 4: Healthier Work Habits

Without tracking, you don't realize:
- You worked 6 hours straight without standing up
- You're on your computer at 1 AM again
- You switch between apps 200 times per hour
- Your most productive hours are actually 9-11 AM

CompScan surfaces these patterns gently, without judgment.

### Benefit 5: Never Lose Uncommitted Code

How many times have you lost work because a git repo had uncommitted changes? CompScan monitors your repositories and warns you when you have unsaved work.

### Benefit 6: Complete Privacy

Unlike every other monitoring tool:
- Zero data leaves your machine. **Ever.**
- No account to create
- No cloud sync
- No telemetry
- Everything encrypted on disk
- You own 100% of your data

### Benefit 7: Zero Cost

Free forever. Open source. No premium tier. No "upgrade to unlock." Everything works, always.

---

## 5. Why Should I Use This Every Day?

### The Compound Effect

CompScan is like brushing your teeth — each individual session seems small, but the compound effect is massive:

| Timeframe | What CompScan Does For You |
|-----------|---------------------------|
| **Day 1** | Finds 5-10 issues you didn't know about. Recovers several GB of disk. |
| **Week 1** | Learns your patterns. Notices your peak productivity hours. |
| **Month 1** | Has a full behavioral profile. Catches anomalies instantly. |
| **Month 3** | You have a measurably faster machine, better habits, and zero security gaps. |

### It Gets Smarter Over Time

The more data CompScan collects, the better its insights become:
- Statistical anomaly detection improves with more baseline data
- Pattern recognition identifies your personal rhythms
- AI analysis (with Ollama) can reason about long-term trends

### It Runs Itself

After initial setup, CompScan requires almost zero effort:
1. Start the observer once (`compscan observe`) — leave it running
2. Glance at `compscan status` when you want
3. Run `compscan report` when you're curious
4. Click to fix anything it finds

**The work-to-benefit ratio is extremely high.** 30 seconds of input gives you insights that would take hours to discover manually.

### It Protects You Silently

Even when you forget about it:
- It's monitoring for memory leaks
- It's catching security regressions
- It's noticing when your disk is filling up
- It's tracking your uncommitted code

You only hear from it when something actually matters.

---

## 6. Real Scenarios Where CompScan Saves You

### Scenario: "Why is my laptop so slow?"

```bash
$ compscan scan

Resource hogs detected:
  - Google Chrome (PID 1234) | CPU: 45% | Mem: 2.3 GB
  - Docker Desktop (PID 5678) | CPU: 15% | Mem: 1.8 GB

$ compscan report

WARNINGS:
  [Memory] High memory usage: 92%
  [Performance] Heavy swap usage: 78%
  → System is swapping to disk, causing slowdowns.
  → Close Chrome tabs or restart Docker to free 4 GB.
```

**Result:** You close 30 Chrome tabs you forgot about. Machine is fast again.

### Scenario: "I'm running out of disk space"

```bash
$ compscan scan --full

Cache dirs: 8.3 GB reclaimable
  1.  3.2 GB  ~/Library/Caches
  2.  2.1 GB  ~/.npm
  3.  1.8 GB  ~/Library/Developer/Xcode/DerivedData
  4.  1.2 GB  ~/.cargo/registry

Large files (>100MB):
  1.  2.4 GB  ~/Downloads/ubuntu-22.04.iso
  2.  1.1 GB  ~/old-project/node_modules/...

$ compscan act cleanup-caches    # Frees 8+ GB instantly
```

**Result:** 10 GB recovered without losing anything important.

### Scenario: "Did I commit my work before the weekend?"

```bash
$ compscan report

CODING:
  3 repo(s) with uncommitted changes:
  - client-app (12 changes)
  - api-server (3 changes)
  - scripts (1 change)
  → Consider committing or stashing your work.
```

**Result:** You save hours of reconstruction on Monday.

### Scenario: "Am I burning out?"

```bash
$ compscan report

HABITS:
  Late night computer usage detected (1:30 AM)
  → Consider winding down. Enable night mode.

  Extended work session detected (3+ hours without break)
  → Take a 5-minute break. Look away, stretch, hydrate.
```

**Result:** You actually take a break. Your eyes thank you.

---

## 7. Command Reference

| Command | What It Does | When To Use |
|---------|-------------|-------------|
| `compscan scan` | Full system health check | First time, weekly, or when things feel slow |
| `compscan scan --full` | Deep scan including large file detection | Weekly or when low on disk |
| `compscan observe` | Start background monitoring | Daily — leave running while you work |
| `compscan report` | Generate insights from all collected data | End of day, or anytime you're curious |
| `compscan report --json` | Machine-readable report | For scripting or piping to other tools |
| `compscan dashboard` | Interactive terminal UI | When you want a live overview |
| `compscan web` | Browser dashboard at localhost:7890 | When you prefer a visual interface |
| `compscan act <action>` | Execute a fix | When the report suggests something |
| `compscan status` | Quick system summary | Morning check-in (10 seconds) |
| `compscan config` | View settings | When you want to tune behavior |

### Built-in Actions

| Action | Risk | What It Does |
|--------|------|-------------|
| `cleanup-caches` | Low | Removes old cached files (safe, frees space) |
| `cleanup-temp` | Low | Cleans temp files older than 7 days |
| `cleanup-logs` | Low | Removes log files older than 30 days |
| `cleanup-disk` | Medium | Full cleanup including npm, pip, cargo caches |
| `kill-top-memory` | High | Terminates the highest memory process |
| `optimize-startup` | Medium | Reviews and suggests disabling startup items |

---

## 8. FAQ

**Q: Does it slow down my computer?**
No. CompScan is built in Rust and uses minimal resources. The observer samples once every 30 seconds and takes less than 200ms each time. The release binary is only 3 MB.

**Q: Does it send any data anywhere?**
No. Zero bytes leave your machine. The only network connection is to a local Ollama instance (optional, on localhost only). There is no telemetry, no analytics, no cloud.

**Q: What if I don't have Ollama?**
CompScan works perfectly without it. The built-in rule engine (30+ rules) and statistical analyzer handle the vast majority of insights. Ollama adds deeper reasoning but is entirely optional.

**Q: Can it break my system?**
No. Every action has a risk level. Low-risk actions only touch temp files and caches. Medium-risk actions ask for confirmation. High-risk actions require you to type "yes." Critical actions are blocked entirely — CompScan will never touch system files automatically.

**Q: Where is my data stored?**
- macOS: `~/Library/Application Support/compscan/`
- Linux: `~/.local/share/compscan/`
- Windows: `%APPDATA%/compscan/`

All in a single database file. Delete it anytime to start fresh.

**Q: Can I use this at work?**
Absolutely. CompScan runs locally and doesn't interact with any external service. No IT policy violation. No data exfiltration risk.

**Q: How do I update?**
```bash
cd comp-scan && git pull && cargo install --path .
```

**Q: How do I uninstall?**
```bash
cargo uninstall compscan
rm -rf ~/Library/Application\ Support/compscan/   # macOS
# or: rm -rf ~/.local/share/compscan/              # Linux
```
