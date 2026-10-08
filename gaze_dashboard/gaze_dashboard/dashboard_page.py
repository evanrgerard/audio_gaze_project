DASHBOARD_HTML = """
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>algo_gaze Dashboard</title>
<style>
  :root {
    --bg: #0f1115; --panel: #171a21; --border: #262b36; --text: #e6e8ec;
    --muted: #8b93a3; --accent: #4da3ff; --good: #35d07f; --warn: #f4b942; --bad: #ff5f56;
  }
  * { box-sizing: border-box; }
  body {
    margin: 0; font-family: -apple-system, Segoe UI, Roboto, sans-serif;
    background: var(--bg); color: var(--text);
  }
  header {
    padding: 14px 20px; border-bottom: 1px solid var(--border);
    display: flex; align-items: center; gap: 16px;
  }
  header h1 { font-size: 18px; margin: 0; }
  header .pill {
    font-size: 12px; padding: 3px 10px; border-radius: 12px;
    background: var(--panel); border: 1px solid var(--border); color: var(--muted);
  }
  .grid {
    display: grid; gap: 14px; padding: 16px;
    grid-template-columns: 1.1fr 1fr 1fr;
    grid-template-areas:
      "video config audio"
      "video people audio"
      "map map audio"
      "exp exp exp"
      "log log log";
  }
  .panel {
    background: var(--panel); border: 1px solid var(--border); border-radius: 10px;
    padding: 14px; overflow: hidden;
  }
  .panel h2 {
    font-size: 13px; text-transform: uppercase; letter-spacing: 0.06em;
    color: var(--muted); margin: 0 0 10px 0;
  }
  #video-panel { grid-area: video; }
  #config-panel { grid-area: config; }
  #audio-panel { grid-area: audio; }
  #people-panel { grid-area: people; }
  #log-panel { grid-area: log; }

  #video-panel img { width: 100%; border-radius: 6px; background: #000; display: block; }
  #map-panel { grid-area: map; }
  #map-canvas { width: 100%; height: 320px; background: #14171f; border-radius: 6px; display: block; }
  .kv { display: flex; justify-content: space-between; padding: 4px 0; font-size: 13px; border-bottom: 1px dashed var(--border); }
  .kv:last-child { border-bottom: none; }
  .kv .k { color: var(--muted); }
  .kv .v { font-weight: 600; }

  table { width: 100%; border-collapse: collapse; font-size: 12px; }
  th, td { text-align: left; padding: 5px 6px; border-bottom: 1px solid var(--border); }
  th { color: var(--muted); font-weight: 500; }
  tr.target-row { background: rgba(77, 163, 255, 0.12); }
  .badge { display: inline-block; padding: 1px 6px; border-radius: 8px; font-size: 11px; }
  .badge.on { background: rgba(53,208,127,0.18); color: var(--good); }
  .badge.off { background: rgba(139,147,163,0.18); color: var(--muted); }

  #log-panel { max-height: 260px; overflow-y: auto; font-family: ui-monospace, monospace; font-size: 12px; }
  .log-line { padding: 2px 0; white-space: pre-wrap; }
  .log-INFO { color: var(--text); }
  .log-WARN { color: var(--warn); }
  .log-ERROR, .log-FATAL { color: var(--bad); }
  .log-node { color: var(--accent); }

  .dot { display:inline-block; width:8px; height:8px; border-radius:50%; margin-right:6px; }
  .dot.good { background: var(--good); } .dot.bad { background: var(--bad); }

  #audio-history { max-height: 180px; overflow-y: auto; margin-top: 10px; font-size: 12px; }

  #exp-panel { grid-area: exp; }
  .exp-row { display: flex; gap: 16px; align-items: flex-start; }
  .exp-controls { flex: 0 0 220px; }
  .exp-list {
    flex: 0 0 240px; max-height: 240px; overflow-y: auto;
    border-right: 1px solid var(--border); padding-right: 14px;
  }
  .exp-detail { flex: 1; min-width: 0; }
  .exp-item {
    padding: 6px 8px; border-radius: 6px; cursor: pointer; font-size: 12px;
    font-family: ui-monospace, monospace; color: var(--text);
  }
  .exp-item:hover { background: rgba(77,163,255,0.1); }
  .exp-item.selected { background: rgba(77,163,255,0.22); }
  .exp-detail h3 {
    font-size: 11px; text-transform: uppercase; letter-spacing: 0.05em;
    color: var(--muted); margin: 14px 0 6px 0;
  }
  .exp-detail h3:first-child { margin-top: 0; }
  button.rec-btn {
    width: 100%; padding: 10px; border-radius: 8px; border: 1px solid var(--border);
    background: var(--panel); color: var(--text); font-size: 13px; font-weight: 600;
    cursor: pointer;
  }
  button.rec-btn.recording {
    background: rgba(255,95,86,0.18); border-color: var(--bad); color: var(--bad);
  }
</style>
</head>
<body>
<header>
  <h1>algo_gaze Dashboard</h1>
  <span class="pill" id="conn-status"><span class="dot bad"></span>connecting...</span>
</header>

<div class="grid">

  <div class="panel" id="video-panel">
    <h2>Live Annotated Feed</h2>
    <img src="/video_feed" alt="annotated feed">
  </div>

  <div class="panel" id="config-panel">
    <h2>Configuration</h2>
    <div class="kv"><span class="k">mode</span><span class="v" id="cfg-mode">-</span></div>
    <div class="kv"><span class="k">audio_backend</span><span class="v" id="cfg-audio-backend">-</span></div>
    <div class="kv"><span class="k">FPS</span><span class="v" id="stat-fps">-</span></div>
    <div class="kv"><span class="k">Latency (ms)</span><span class="v" id="stat-latency">-</span></div>
    <div class="kv"><span class="k">Recording (CSV)</span><span class="v" id="stat-recording">-</span></div>
    <div class="kv"><span class="k">Pan (commanded)</span><span class="v" id="stat-pan-cmd">-</span></div>
    <div class="kv"><span class="k">Tilt (commanded)</span><span class="v" id="stat-tilt-cmd">-</span></div>
    <div class="kv"><span class="k">Pan (actual, sim)</span><span class="v" id="stat-pan-act">-</span></div>
    <div class="kv"><span class="k">Tilt (actual, sim)</span><span class="v" id="stat-tilt-act">-</span></div>
  </div>

  <div class="panel" id="audio-panel">
    <h2>Audio Cue</h2>
    <div class="kv"><span class="k">Active</span><span class="v" id="audio-active">-</span></div>
    <div class="kv"><span class="k">Direction (deg)</span><span class="v" id="audio-dir">-</span></div>
    <div class="kv"><span class="k">Confidence</span><span class="v" id="audio-conf">-</span></div>
    <div class="kv"><span class="k">Matched a person</span><span class="v" id="audio-matched">-</span></div>
    <h2 style="margin-top:14px;">History (recent bursts)</h2>
    <div id="audio-history"></div>
  </div>

  <div class="panel" id="people-panel">
    <h2>Detected People &amp; Fuzzy Scores</h2>
    <table id="people-table">
      <thead><tr><th>ID</th><th>Score</th><th>Prox.</th><th>Speech</th><th>Point</th><th>Wave</th></tr></thead>
      <tbody></tbody>
    </table>
  </div>

  <div class="panel" id="map-panel">
    <h2>Top-Down Map (ground truth, Webots sim)</h2>
    <div style="font-size:11px; color:var(--muted); margin-bottom:8px;">
      <span style="color:#35d07f;">&#9679;</span> robot / camera heading &amp; FOV &nbsp;
      <span style="color:#f4b942;">&#9679;</span> person (idle) &nbsp;
      <span style="color:#ff5f56;">&#9679;</span> person (ground-truth "speaking") &nbsp;
      <span style="color:#c864ff;">&#9679;</span> raw /audio/cue direction (independent of any person)
    </div>
    <canvas id="map-canvas" width="900" height="320"></canvas>
  </div>

  <div class="panel" id="exp-panel">
    <h2>Experiments (rosbag2)</h2>
    <div class="exp-row">
      <div class="exp-controls">
        <button class="rec-btn" id="rec-btn" onclick="toggleRecording()">
          &#9679; Start Recording
        </button>
        <div class="kv"><span class="k">Status</span>
          <span class="v" id="rec-status">idle</span></div>
        <div class="kv"><span class="k">Bag</span><span class="v" id="rec-bag">-</span></div>
        <div class="kv"><span class="k">Elapsed</span>
          <span class="v" id="rec-elapsed">-</span></div>
      </div>
      <div class="exp-list" id="exp-list">
        <div style="color:var(--muted); font-size:12px;">No experiments yet</div>
      </div>
      <div class="exp-detail" id="exp-detail">
        <div style="color:var(--muted); font-size:12px;">
          Select an experiment to see its metrics
        </div>
      </div>
    </div>
  </div>

  <div class="panel" id="log-panel"></div>

</div>

<script>
function drawMap(world, audioCurrent) {
  const canvas = document.getElementById('map-canvas');
  const ctx = canvas.getContext('2d');
  const W = canvas.width, H = canvas.height;
  ctx.clearRect(0, 0, W, H);

  if (!world || !world.robot) {
    ctx.fillStyle = '#8b93a3'; ctx.font = '13px sans-serif';
    ctx.fillText('No world position data (audio_backend must be webots_ground_truth)', 14, H / 2);
    return;
  }

  const robot = world.robot;
  const people = world.people || [];

  // Fit all points (robot + people) into the canvas with padding, world Y flipped
  // so "up" on screen roughly matches "forward" for a robot facing +X initially.
  const xs = [robot.x, ...people.map(p => p.x)];
  const ys = [robot.y, ...people.map(p => p.y)];
  const pad = 1.0; // meters of margin around the bounding box
  const minX = Math.min(...xs) - pad, maxX = Math.max(...xs) + pad;
  const minY = Math.min(...ys) - pad, maxY = Math.max(...ys) + pad;
  const spanX = Math.max(maxX - minX, 0.5), spanY = Math.max(maxY - minY, 0.5);
  const scale = Math.min(W / spanX, H / spanY) * 0.9;
  const offX = W / 2 - ((minX + maxX) / 2) * scale;
  const offY = H / 2 + ((minY + maxY) / 2) * scale; // + because we flip Y below

  const toScreen = (x, y) => [x * scale + offX, offY - y * scale];

  // Grid
  ctx.strokeStyle = '#262b36'; ctx.lineWidth = 1;
  for (let gx = Math.floor(minX); gx <= Math.ceil(maxX); gx++) {
    const [sx1, sy1] = toScreen(gx, minY), [sx2, sy2] = toScreen(gx, maxY);
    ctx.beginPath(); ctx.moveTo(sx1, sy1); ctx.lineTo(sx2, sy2); ctx.stroke();
  }
  for (let gy = Math.floor(minY); gy <= Math.ceil(maxY); gy++) {
    const [sx1, sy1] = toScreen(minX, gy), [sx2, sy2] = toScreen(maxX, gy);
    ctx.beginPath(); ctx.moveTo(sx1, sy1); ctx.lineTo(sx2, sy2); ctx.stroke();
  }

  const [rx, ry] = toScreen(robot.x, robot.y);

  // Dashed lines + angle labels from robot to each person
  people.forEach(p => {
    const [px, py] = toScreen(p.x, p.y);
    ctx.setLineDash([5, 4]);
    ctx.strokeStyle = p.speaking ? '#ff5f56' : '#4da3ff';
    ctx.lineWidth = p.speaking ? 2 : 1;
    ctx.beginPath(); ctx.moveTo(rx, ry); ctx.lineTo(px, py); ctx.stroke();
    ctx.setLineDash([]);

    const bearingDeg = (Math.atan2(p.y - robot.y, p.x - robot.x) * 180 / Math.PI - robot.yaw_deg + 360) % 360;
    const labelBearing = bearingDeg > 180 ? (bearingDeg - 360).toFixed(0) : bearingDeg.toFixed(0);
    ctx.fillStyle = p.speaking ? '#ff5f56' : '#8b93a3';
    ctx.font = '11px sans-serif';
    ctx.fillText(`${labelBearing}°`, (rx + px) / 2 + 4, (ry + py) / 2 - 4);

    // Person marker
    ctx.beginPath();
    ctx.arc(px, py, p.speaking ? 9 : 7, 0, Math.PI * 2);
    ctx.fillStyle = p.speaking ? '#ff5f56' : '#f4b942';
    ctx.fill();
    ctx.fillStyle = '#e6e8ec'; ctx.font = '11px sans-serif';
    ctx.fillText(p.name, px + 10, py + 4);
  });

  // Camera field-of-view cone (dashed), using camera_heading_deg
  const fovHalfDeg = 39; // half of the ~78 deg default camera_hfov_deg param
  const coneLenPx = Math.min(W, H) * 0.45;
  [-fovHalfDeg, fovHalfDeg].forEach(offsetDeg => {
    const angleRad = (robot.camera_heading_deg + offsetDeg) * Math.PI / 180;
    ctx.setLineDash([3, 3]);
    ctx.strokeStyle = '#35d07f'; ctx.lineWidth = 1;
    ctx.beginPath(); ctx.moveTo(rx, ry);
    ctx.lineTo(rx + Math.cos(angleRad) * coneLenPx, ry - Math.sin(angleRad) * coneLenPx);
    ctx.stroke();
    ctx.setLineDash([]);
  });

  // Raw /audio/cue ray -- independent of any person, this is what the audio
  // system ACTUALLY reported. With a real (non-ground-truth) audio backend,
  // this will NOT necessarily line up with a person marker -- that mismatch
  // is exactly what this ray is for showing.
  if (audioCurrent && audioCurrent.is_speech) {
    const worldAngleDeg = robot.camera_heading_deg + audioCurrent.direction_deg;
    const angleRad = worldAngleDeg * Math.PI / 180;
    const rayLenPx = Math.min(W, H) * 0.42;
    const conf = audioCurrent.confidence != null ? audioCurrent.confidence : 1.0;

    ctx.strokeStyle = `rgba(200, 100, 255, ${0.35 + 0.65 * conf})`; // purple, fades with low confidence
    ctx.lineWidth = 3;
    ctx.setLineDash([]);
    ctx.beginPath(); ctx.moveTo(rx, ry);
    ctx.lineTo(rx + Math.cos(angleRad) * rayLenPx, ry - Math.sin(angleRad) * rayLenPx);
    ctx.stroke();

    ctx.fillStyle = '#c864ff'; ctx.font = 'bold 11px sans-serif';
    ctx.fillText(
      `AUDIO ${audioCurrent.direction_deg.toFixed(0)}° (conf ${conf.toFixed(2)})`,
      rx + Math.cos(angleRad) * rayLenPx * 0.6 + 6,
      ry - Math.sin(angleRad) * rayLenPx * 0.6 - 6
    );
  }

  // Robot marker (star-ish) + heading arrow (solid, camera direction)
  const headingRad = robot.camera_heading_deg * Math.PI / 180;
  ctx.strokeStyle = '#35d07f'; ctx.lineWidth = 2;
  ctx.beginPath(); ctx.moveTo(rx, ry);
  ctx.lineTo(rx + Math.cos(headingRad) * 40, ry - Math.sin(headingRad) * 40);
  ctx.stroke();
  ctx.beginPath();
  ctx.arc(rx, ry, 8, 0, Math.PI * 2);
  ctx.fillStyle = '#35d07f'; ctx.fill();
  ctx.fillStyle = '#e6e8ec'; ctx.font = '11px sans-serif';
  ctx.fillText('BRONE', rx + 10, ry - 10);
}

let _wasRecording = false;

async function toggleRecording() {
  const btn = document.getElementById('rec-btn');
  btn.disabled = true;
  try {
    const recording = btn.classList.contains('recording');
    const url = recording ? '/api/record/stop' : '/api/record/start';
    const res = await fetch(url, { method: 'POST' });
    const body = await res.json();
    if (!res.ok) alert('Recording error: ' + (body.error || res.status));
  } catch (e) {
    alert('Recording request failed: ' + e);
  } finally {
    btn.disabled = false;
  }
}

function fmtVal(v, digits) {
  if (v === null || v === undefined) return '-';
  if (typeof v === 'number') return v.toFixed(digits ?? 3);
  return String(v);
}

function kvRow(label, value) {
  return `<div class="kv"><span class="k">${label}</span><span class="v">${value}</span></div>`;
}

function renderStatsRow(label, stats, digits) {
  if (!stats) return '';
  const v = `min ${fmtVal(stats.min, digits)} / avg ${fmtVal(stats.avg, digits)} / `
    + `max ${fmtVal(stats.max, digits)} (n=${stats.n})`;
  return kvRow(label, v);
}

async function loadExperiments() {
  const res = await fetch('/api/experiments');
  const exps = await res.json();
  const listDiv = document.getElementById('exp-list');
  listDiv.innerHTML = exps.length
    ? exps.map(e => `<div class="exp-item" data-name="${e.name}" `
        + `onclick="viewExperiment('${e.name}')">${e.name}</div>`).join('')
    : '<div style="color:var(--muted); font-size:12px;">No experiments yet</div>';
}

async function viewExperiment(name) {
  document.querySelectorAll('.exp-item').forEach(el => {
    el.classList.toggle('selected', el.dataset.name === name);
  });
  const detail = document.getElementById('exp-detail');
  detail.innerHTML = '<div style="color:var(--muted); font-size:12px;">Loading...</div>';
  try {
    const res = await fetch(`/api/experiments/${name}`);
    const s = await res.json();
    if (!res.ok) {
      const msg = s.error || res.status;
      detail.innerHTML = `<div style="color:var(--bad); font-size:12px;">${msg}</div>`;
      return;
    }
    const c = s.condition || {};
    const frames = `${s.gaze_metrics_frames ?? 0} / ${s.joint_state_samples ?? 0}`;
    const cond = `sim=${c.simulation_mode ?? '-'} audio_mode=${c.audio_mode ?? '-'} `
      + `mic_mount=${c.mic_mount ?? '-'}`;
    let html = '<h3>Run</h3>'
      + kvRow('Duration', fmtVal(s.duration_sec, 1) + ' s')
      + kvRow('Frames / joint samples', frames)
      + kvRow('Condition', cond);
    if (s.gaze) {
      html += '<h3>Gaze performance</h3>'
        + renderStatsRow('FPS', s.gaze.fps, 1)
        + renderStatsRow('Latency (ms)', s.gaze.latency_ms, 1)
        + renderStatsRow('Pixel error', s.gaze.error_px, 1)
        + kvRow('Time on target', fmtVal(s.gaze.time_on_target_pct, 1) + '%')
        + renderStatsRow('Gaze shift time (ms)', s.gaze.gaze_shift_time_ms, 0);
    }
    if (s.command_vs_actual) {
      const cva = s.command_vs_actual;
      html += '<h3>Command vs actual (motor tracking)</h3>'
        + kvRow('Pan RMSE (rad)', fmtVal(cva.pan_rmse_rad, 4))
        + kvRow('Tilt RMSE (rad)', fmtVal(cva.tilt_rmse_rad, 4));
    }
    if (s.audio) {
      html += '<h3>Audio</h3>'
        + kvRow('Active', fmtVal(s.audio.active_pct, 1) + '% of frames')
        + kvRow('Matched a person', fmtVal(s.audio.matched_pct, 1) + '% of frames');
    }
    detail.innerHTML = html;
  } catch (e) {
    detail.innerHTML = `<div style="color:var(--bad); font-size:12px;">${e}</div>`;
  }
}

async function poll() {
  try {
    const res = await fetch('/api/state');
    const data = await res.json();
    document.getElementById('conn-status').innerHTML = '<span class="dot good"></span>connected';

    document.getElementById('cfg-mode').textContent = data.config.mode || '(not set)';
    document.getElementById('cfg-audio-backend').textContent = data.config.audio_backend || '(not set)';

    const g = data.gaze || {};
    document.getElementById('stat-fps').textContent = g.fps ?? '-';
    document.getElementById('stat-latency').textContent = g.latency_ms != null ? g.latency_ms + ' ms' : '-';
    document.getElementById('stat-recording').textContent = g.is_recording ? 'ON' : 'off';
    document.getElementById('stat-pan-cmd').textContent = g.pan_deg != null ? g.pan_deg + ' deg' : '-';
    document.getElementById('stat-tilt-cmd').textContent = g.tilt_deg != null ? g.tilt_deg + ' deg' : '-';

    const pa = data.pan_tilt_actual || {};
    document.getElementById('stat-pan-act').textContent = pa.pan_deg != null ? pa.pan_deg + ' deg' : '-';
    document.getElementById('stat-tilt-act').textContent = pa.tilt_deg != null ? pa.tilt_deg + ' deg' : '-';

    const a = data.audio_current;
    document.getElementById('audio-active').innerHTML = a && a.is_speech
      ? '<span class="badge on">SPEAKING</span>' : '<span class="badge off">silent</span>';
    document.getElementById('audio-dir').textContent = a ? a.direction_deg + ' deg' : '-';
    document.getElementById('audio-conf').textContent = a ? a.confidence : '-';
    document.getElementById('audio-matched').textContent = (g.audio && g.audio.matched_person) ? 'yes' : 'no';

    const histDiv = document.getElementById('audio-history');
    histDiv.innerHTML = (data.audio_history || []).slice().reverse().map(h => {
      const t = new Date(h.stamp * 1000).toLocaleTimeString();
      const label = h.is_speech ? 'START' : 'END  ';
      return `<div>${t} [${label}] dir=${h.direction_deg} deg conf=${h.confidence}</div>`;
    }).join('');

    drawMap(data.world_positions, data.audio_current);

    const rec = data.recording || {};
    const btn = document.getElementById('rec-btn');
    btn.classList.toggle('recording', !!rec.active);
    btn.textContent = rec.active ? '■ Stop Recording' : '● Start Recording';
    document.getElementById('rec-status').textContent = rec.active ? 'RECORDING' : 'idle';
    document.getElementById('rec-bag').textContent = rec.bag_name || '-';
    document.getElementById('rec-elapsed').textContent =
      rec.elapsed_sec != null ? rec.elapsed_sec.toFixed(0) + ' s' : '-';
    if (_wasRecording && !rec.active) loadExperiments();  // just stopped -- refresh the list
    _wasRecording = !!rec.active;

    const tbody = document.querySelector('#people-table tbody');
    const targetId = g.target_id;
    tbody.innerHTML = (g.people || []).map(p => {
      const c = p.cues || {};
      const isTarget = p.id === targetId;
      return `<tr class="${isTarget ? 'target-row' : ''}">
        <td>${p.id}${isTarget ? ' ★' : ''}</td>
        <td>${p.score}</td>
        <td>${c.proximity ?? '-'}</td>
        <td>${c.speech ? '<span class="badge on">yes</span>' : '<span class="badge off">no</span>'}</td>
        <td>${c.pointing ?? '-'}</td>
        <td>${c.waving ?? '-'}</td>
      </tr>`;
    }).join('') || '<tr><td colspan="6" style="color:var(--muted)">No people detected</td></tr>';

    const logPanel = document.getElementById('log-panel');
    const wasScrolledToBottom = logPanel.scrollHeight - logPanel.clientHeight <= logPanel.scrollTop + 30;
    logPanel.innerHTML = '<h2>Live Log (algo_gaze, audio, webots controller)</h2>' +
      (data.logs || []).map(l => {
        const t = new Date(l.stamp * 1000).toLocaleTimeString();
        return `<div class="log-line log-${l.level}"><span class="log-node">[${l.node}]</span> ${t} ${l.level}: ${l.msg}</div>`;
      }).join('');
    if (wasScrolledToBottom) logPanel.scrollTop = logPanel.scrollHeight;

  } catch (e) {
    document.getElementById('conn-status').innerHTML = '<span class="dot bad"></span>disconnected';
  }
}
setInterval(poll, 400);
poll();
loadExperiments();
</script>
</body>
</html>
"""
