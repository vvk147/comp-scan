pub fn index_html() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>CompScan Dashboard</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
:root{--bg:#0d1117;--card:#161b22;--border:#30363d;--text:#c9d1d9;--dim:#8b949e;
--cyan:#58a6ff;--green:#3fb950;--yellow:#d29922;--red:#f85149;--purple:#bc8cff}
body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;background:var(--bg);color:var(--text);padding:1rem}
h1{color:var(--cyan);margin-bottom:0.5rem;font-size:1.5rem}
.subtitle{color:var(--dim);margin-bottom:1.5rem;font-size:0.9rem}
.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));gap:1rem;margin-bottom:1rem}
.card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:1rem}
.card h2{font-size:1rem;color:var(--cyan);margin-bottom:0.75rem;border-bottom:1px solid var(--border);padding-bottom:0.5rem}
.stat{display:flex;justify-content:space-between;padding:0.3rem 0;border-bottom:1px solid var(--border)}
.stat:last-child{border-bottom:none}
.stat .label{color:var(--dim)}
.stat .value{font-weight:600}
.bar-container{background:#21262d;border-radius:4px;height:8px;margin:4px 0;overflow:hidden}
.bar{height:100%;border-radius:4px;transition:width 0.3s}
.bar.green{background:var(--green)}.bar.yellow{background:var(--yellow)}.bar.red{background:var(--red)}
table{width:100%;border-collapse:collapse;font-size:0.85rem}
th{text-align:left;color:var(--cyan);padding:0.5rem;border-bottom:2px solid var(--border)}
td{padding:0.4rem 0.5rem;border-bottom:1px solid var(--border)}
.badge{display:inline-block;padding:2px 8px;border-radius:12px;font-size:0.75rem;font-weight:600}
.badge.critical{background:var(--red);color:#fff}
.badge.warning{background:var(--yellow);color:#000}
.badge.suggestion{background:var(--cyan);color:#000}
.badge.info{background:var(--purple);color:#fff}
.empty{color:var(--dim);text-align:center;padding:2rem;font-style:italic}
#status-dot{display:inline-block;width:8px;height:8px;border-radius:50%;margin-right:6px}
#status-dot.online{background:var(--green)}#status-dot.offline{background:var(--red)}
</style>
</head>
<body>
<h1>CompScan Dashboard</h1>
<p class="subtitle"><span id="status-dot" class="online"></span><span id="status-text">Loading...</span>
  <span style="margin-left:1rem;font-size:0.8rem;">
    <a href="https://github.com/vvk147/comp-scan" target="_blank" rel="noopener" style="color:var(--cyan);text-decoration:none;">★ Star</a>
    ·
    <a href="https://github.com/sponsors/vvk147" target="_blank" rel="noopener" style="color:var(--cyan);text-decoration:none;">Sponsor</a>
  </span>
</p>

<div class="grid">
  <div class="card" id="system-card"><h2>System</h2><div id="system-info" class="empty">Loading...</div></div>
  <div class="card" id="resources-card"><h2>Resources</h2><div id="resources-info" class="empty">Loading...</div></div>
</div>

<div class="grid">
  <div class="card"><h2>Insights</h2><div id="insights-list" class="empty">No insights yet</div></div>
  <div class="card"><h2>Recent Activity</h2><div id="activity-list" class="empty">No activity recorded</div></div>
</div>

<script>
const API = '';
async function fetchJSON(url) { const r = await fetch(url); if(!r.ok) return null; return r.json(); }

function pctColor(p) { return p > 90 ? 'red' : p > 75 ? 'yellow' : 'green'; }
function bar(pct) { return `<div class="bar-container"><div class="bar ${pctColor(pct)}" style="width:${Math.min(pct,100)}%"></div></div>`; }
function fmtBytes(b) { if(b>1e9) return (b/1e9).toFixed(1)+'GB'; if(b>1e6) return (b/1e6).toFixed(0)+'MB'; return (b/1e3).toFixed(0)+'KB'; }

async function loadStatus() {
  const s = await fetchJSON('/api/status');
  if(!s) { document.getElementById('status-text').textContent='Disconnected'; document.getElementById('status-dot').className='offline'; return; }
  document.getElementById('status-text').textContent = `v${s.version} | ${s.hostname} | ${s.os} | ${s.activity_count} activities | ${s.insight_count} insights`;
}

async function loadSnapshot() {
  const s = await fetchJSON('/api/snapshot');
  const el = document.getElementById('system-info');
  if(!s) { el.innerHTML='<p class="empty">Run compscan scan first</p>'; return; }
  const memPct = (s.used_memory_bytes/s.total_memory_bytes*100).toFixed(0);
  el.innerHTML = `
    <div class="stat"><span class="label">Host</span><span class="value">${s.hostname}</span></div>
    <div class="stat"><span class="label">OS</span><span class="value">${s.os_name} ${s.os_version}</span></div>
    <div class="stat"><span class="label">CPU</span><span class="value">${s.cpu_brand} (${s.cpu_count} cores)</span></div>
    <div class="stat"><span class="label">Processes</span><span class="value">${s.process_count}</span></div>`;
  const rEl = document.getElementById('resources-info');
  let html = `<div class="stat"><span class="label">Memory ${memPct}%</span><span class="value">${fmtBytes(s.used_memory_bytes)} / ${fmtBytes(s.total_memory_bytes)}</span></div>${bar(memPct)}`;
  for(const d of s.disks||[]) {
    const used=d.total_bytes-d.available_bytes; const pct=(used/d.total_bytes*100).toFixed(0);
    html += `<div class="stat"><span class="label">${d.mount_point} ${pct}%</span><span class="value">${fmtBytes(d.available_bytes)} free</span></div>${bar(pct)}`;
  }
  rEl.innerHTML = html;
}

async function loadInsights() {
  const data = await fetchJSON('/api/insights');
  const el = document.getElementById('insights-list');
  if(!data||!data.length) { el.innerHTML='<p class="empty">No insights. Run compscan report.</p>'; return; }
  el.innerHTML = '<table><tr><th>Sev</th><th>Category</th><th>Insight</th></tr>' +
    data.slice(0,15).map(i => `<tr><td><span class="badge ${i.severity.toLowerCase()}">${i.severity}</span></td><td>${i.category}</td><td>${i.title}</td></tr>`).join('') + '</table>';
}

async function loadActivity() {
  const data = await fetchJSON('/api/activities');
  const el = document.getElementById('activity-list');
  if(!data||!data.length) { el.innerHTML='<p class="empty">Run compscan observe first.</p>'; return; }
  el.innerHTML = '<table><tr><th>Time</th><th>CPU</th><th>Mem</th><th>Top Process</th></tr>' +
    data.slice(0,15).map(a => {
      const t = new Date(a.timestamp).toLocaleTimeString();
      return `<tr><td>${t}</td><td>${a.cpu_usage_percent.toFixed(0)}%</td><td>${a.memory_usage_percent.toFixed(0)}%</td><td>${a.top_cpu_process}</td></tr>`;
    }).join('') + '</table>';
}

async function refresh() { await Promise.all([loadStatus(), loadSnapshot(), loadInsights(), loadActivity()]); }
refresh();
setInterval(refresh, 5000);
</script>
<footer style="margin-top:1.5rem;padding-top:0.75rem;border-top:1px solid var(--border);color:var(--dim);font-size:0.8rem;">
  <a href="https://github.com/vvk147/comp-scan" target="_blank" rel="noopener" style="color:var(--cyan);text-decoration:none;">Star on GitHub</a> ·
  <a href="https://github.com/sponsors/vvk147" target="_blank" rel="noopener" style="color:var(--cyan);text-decoration:none;">Sponsor</a>
  — CompScan is free and open source. Your support helps keep it maintained.
</footer>
</body>
</html>"##.to_string()
}
