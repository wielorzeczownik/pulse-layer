interface Zone {
  max: number;
  cls: string;
  borderA: number;
  glowA: number;
  glowPx: number;
}

interface Config {
  calm?: string;
  normal?: string;
  high?: string;
  fast?: string;
  alarm?: string;
  panel_bg?: string;
  style?: string;
}

interface WsMessage {
  config?: Config;
  bpm?: number | null;
}

const root = document.documentElement;
const heartEl = document.getElementById("heart-widget") as HTMLElement;
const hwBpm = document.getElementById("hw-bpm") as HTMLElement;
const heartGlyph = document.getElementById("heart") as HTMLElement;
const ecgEl = document.getElementById("ecg-widget") as HTMLElement;
const ecgBpmEl = document.getElementById("ecg-bpm") as HTMLElement;
const ecgPath = document.getElementById(
  "ecg-path"
) as unknown as SVGPathElement;

const ZONES: Zone[] = [
  { max: 64, cls: "calm", borderA: 0.2, glowA: 0, glowPx: 0 },
  { max: 80, cls: "normal", borderA: 0.2, glowA: 0, glowPx: 0 },
  { max: 100, cls: "high", borderA: 0.25, glowA: 0, glowPx: 0 },
  { max: 130, cls: "fast", borderA: 0.35, glowA: 0.15, glowPx: 22 },
  { max: 999, cls: "alarm", borderA: 0.55, glowA: 0.22, glowPx: 38 },
];

const colors: Record<string, string> = {
  calm: "#52C27A",
  normal: "#5B9BD5",
  high: "#E5B950",
  fast: "#E07A30",
  alarm: "#D94545",
};

let currentBpm: number | null = null;
let activeStyle = "heart";

function hexToRgba(hex: string, a: number): string {
  const h = hex.replace("#", "");
  const r = parseInt(h.slice(0, 2), 16);
  const g = parseInt(h.slice(2, 4), 16);
  const b = parseInt(h.slice(4, 6), 16);
  return `rgba(${r},${g},${b},${a})`;
}

function zoneForBpm(bpm: number): Zone {
  return ZONES.find((z) => bpm <= z.max) ?? ZONES[ZONES.length - 1];
}

function applyPanelZone(panel: HTMLElement, zone: Zone): void {
  panel.className = `panel ${zone.cls}`;
  const color = colors[zone.cls];
  panel.style.borderColor = hexToRgba(color, zone.borderA);
  panel.style.boxShadow =
    zone.glowA > 0
      ? `0 0 ${zone.glowPx}px ${hexToRgba(color, zone.glowA)}`
      : "";
  if (panel === ecgEl) ecgPath.style.setProperty("stroke", color);
}

function applyConfig(cfg: Config): void {
  const zoneMap: Record<string, string> = {
    calm: "--calm-color",
    normal: "--normal-color",
    high: "--high-color",
    fast: "--fast-color",
    alarm: "--alarm-color",
  };
  for (const [k, prop] of Object.entries(zoneMap)) {
    const val = cfg[k as keyof Config];
    if (val) {
      colors[k] = val;
      root.style.setProperty(prop, val);
    }
  }
  if (cfg.panel_bg) {
    root.style.setProperty("--panel-bg", hexToRgba(cfg.panel_bg, 0.82));
  }
  if (cfg.style !== undefined) {
    activeStyle = cfg.style;
    heartEl.style.display = activeStyle === "heart" ? "flex" : "none";
    ecgEl.style.display = activeStyle === "pulse" ? "flex" : "none";
  }
  if (currentBpm !== null) applyBpm(currentBpm);
}

function applyBpm(bpm: number | null): void {
  currentBpm = bpm;
  activeStyle === "pulse" ? applyEcgBpm(bpm) : applyHeartBpm(bpm);
}

function applyHeartBpm(bpm: number | null): void {
  if (bpm === null) {
    hwBpm.textContent = "--";
    heartEl.className = "panel off";
    heartEl.style.borderColor = heartEl.style.boxShadow = "";
    return;
  }
  hwBpm.textContent = String(bpm);
  heartGlyph.style.setProperty("--heart-duration", (60 / bpm).toFixed(3) + "s");
  applyPanelZone(heartEl, zoneForBpm(bpm));
}

function applyEcgBpm(bpm: number | null): void {
  if (bpm === null) {
    ecgBpmEl.textContent = "--";
    ecgEl.className = "panel off";
    ecgEl.style.borderColor = ecgEl.style.boxShadow = "";
    return;
  }
  ecgBpmEl.textContent = String(bpm);
  ecgPath.style.setProperty("--ecg-duration", (60 / bpm).toFixed(3) + "s");
  applyPanelZone(ecgEl, zoneForBpm(bpm));
}

function connect(): void {
  const ws = new WebSocket("ws://localhost:9000/ws");
  ws.onmessage = (e: MessageEvent<string>) => {
    try {
      const msg = JSON.parse(e.data) as WsMessage;
      if (msg.config !== undefined) applyConfig(msg.config);
      if (msg.bpm !== undefined) applyBpm(msg.bpm ?? null);
    } catch {
      /* ignore malformed frames */
    }
  };
  ws.onclose = () => {
    applyBpm(null);
    setTimeout(connect, 2000);
  };
  ws.onerror = () => ws.close();
}

connect();
