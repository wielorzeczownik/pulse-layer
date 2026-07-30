export interface Zone {
  max: number;
  cls: string;
  borderA: number;
  glowA: number;
  glowPx: number;
}

export const ZONES: Zone[] = [
  { max: 64, cls: 'calm', borderA: 0.2, glowA: 0, glowPx: 0 },
  { max: 80, cls: 'normal', borderA: 0.2, glowA: 0, glowPx: 0 },
  { max: 100, cls: 'high', borderA: 0.25, glowA: 0, glowPx: 0 },
  { max: 130, cls: 'fast', borderA: 0.35, glowA: 0.15, glowPx: 22 },
  { max: 999, cls: 'alarm', borderA: 0.55, glowA: 0.22, glowPx: 38 },
];

export const DEFAULT_COLORS: Record<string, string> = {
  calm: '#52C27A',
  normal: '#5B9BD5',
  high: '#E5B950',
  fast: '#E07A30',
  alarm: '#D94545',
};

// Same grey the Rust side falls back to, so a malformed value looks identical
// in the settings window and in the overlay.
export const FALLBACK_HEX = '#9999A6';

const HEX_COLOR = /^#?[\da-f]{6}$/i;

export function hexToRgba(hex: string, alpha: number): string {
  const raw = (HEX_COLOR.test(hex) ? hex : FALLBACK_HEX).replace('#', '');
  const red = Number.parseInt(raw.slice(0, 2), 16);
  const green = Number.parseInt(raw.slice(2, 4), 16);
  const blue = Number.parseInt(raw.slice(4, 6), 16);
  return `rgba(${red},${green},${blue},${alpha})`;
}

export function zoneForBpm(bpm: number): Zone {
  return ZONES.find((zone) => bpm <= zone.max) ?? ZONES[ZONES.length - 1];
}
