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
  bpm?: number;
}

const root = document.documentElement;
const heartElement = document.getElementById("heart-widget") as HTMLElement;
const hwBpm = document.getElementById("hw-bpm") as HTMLElement;
const heartGlyph = document.getElementById("heart") as HTMLElement;
const ecgElement = document.getElementById("ecg-widget") as HTMLElement;
const ecgBpmElement = document.getElementById("ecg-bpm") as HTMLElement;
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

let currentBpm: number | undefined;
let activeStyle = "heart";

function hexToRgba(hex: string, alpha: number): string {
  const raw = hex.replace("#", "");
  const red = Number.parseInt(raw.slice(0, 2), 16);
  const green = Number.parseInt(raw.slice(2, 4), 16);
  const blue = Number.parseInt(raw.slice(4, 6), 16);
  return `rgba(${red},${green},${blue},${alpha})`;
}

function zoneForBpm(bpm: number): Zone {
  return ZONES.find((zone) => bpm <= zone.max) ?? ZONES[ZONES.length - 1];
}

function applyPanelZone(panel: HTMLElement, zone: Zone): void {
  panel.className = `panel ${zone.cls}`;
  const color = colors[zone.cls];
  panel.style.borderColor = hexToRgba(color, zone.borderA);
  panel.style.boxShadow =
    zone.glowA > 0
      ? `0 0 ${zone.glowPx}px ${hexToRgba(color, zone.glowA)}`
      : "";
  if (panel === ecgElement) ecgPath.style.setProperty("stroke", color);
}

function applyConfig(cfg: Config): void {
  const zoneMap: Record<string, string> = {
    calm: "--calm-color",
    normal: "--normal-color",
    high: "--high-color",
    fast: "--fast-color",
    alarm: "--alarm-color",
  };
  for (const [key, property] of Object.entries(zoneMap)) {
    const value = cfg[key as keyof Config];
    if (value) {
      colors[key] = value;
      root.style.setProperty(property, value);
    }
  }
  if (cfg.panel_bg) {
    root.style.setProperty("--panel-bg", hexToRgba(cfg.panel_bg, 0.82));
  }
  if (cfg.style !== undefined) {
    activeStyle = cfg.style;
    heartElement.style.display = activeStyle === "heart" ? "flex" : "none";
    ecgElement.style.display = activeStyle === "pulse" ? "flex" : "none";
  }
  if (currentBpm !== undefined) applyBpm(currentBpm);
}

function applyBpm(bpm?: number): void {
  currentBpm = bpm;
  if (activeStyle === "pulse") {
    applyEcgBpm(bpm);
  } else {
    applyHeartBpm(bpm);
  }
}

function applyHeartBpm(bpm?: number): void {
  if (bpm === undefined) {
    hwBpm.textContent = "--";
    heartElement.className = "panel off";
    heartElement.style.borderColor = heartElement.style.boxShadow = "";
    return;
  }
  hwBpm.textContent = String(bpm);
  heartGlyph.style.setProperty("--heart-duration", (60 / bpm).toFixed(3) + "s");
  applyPanelZone(heartElement, zoneForBpm(bpm));
}

function applyEcgBpm(bpm?: number): void {
  if (bpm === undefined) {
    ecgBpmElement.textContent = "--";
    ecgElement.className = "panel off";
    ecgElement.style.borderColor = ecgElement.style.boxShadow = "";
    return;
  }
  ecgBpmElement.textContent = String(bpm);
  ecgPath.style.setProperty("--ecg-duration", (60 / bpm).toFixed(3) + "s");
  applyPanelZone(ecgElement, zoneForBpm(bpm));
}

function connect(): void {
  const ws = new WebSocket("ws://localhost:9000/ws");
  ws.addEventListener("message", (event: MessageEvent<string>) => {
    try {
      const message = JSON.parse(event.data) as WsMessage;
      if (message.config !== undefined) applyConfig(message.config);
      if (message.bpm !== undefined) applyBpm(message.bpm);
    } catch {
      /* ignore malformed frames */
    }
  });
  ws.addEventListener("close", () => {
    applyBpm();
    setTimeout(connect, 2000);
  });
  ws.addEventListener("error", () => {
    ws.close();
  });
}

connect();
