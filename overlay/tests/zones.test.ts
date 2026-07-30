import { describe, expect, it } from 'vitest';

import {
  DEFAULT_COLORS,
  FALLBACK_HEX,
  hexToRgba,
  zoneForBpm,
  ZONES,
} from '../src/zones';

describe('zoneForBpm', () => {
  it('returns the first zone whose ceiling the reading fits under', () => {
    expect(zoneForBpm(50).cls).toBe('calm');
    expect(zoneForBpm(72).cls).toBe('normal');
    expect(zoneForBpm(95).cls).toBe('high');
    expect(zoneForBpm(120).cls).toBe('fast');
    expect(zoneForBpm(180).cls).toBe('alarm');
  });

  it('treats each ceiling as belonging to its own zone', () => {
    for (const zone of ZONES) {
      expect(zoneForBpm(zone.max).cls).toBe(zone.cls);
    }
  });

  it('never falls off the end of the table', () => {
    expect(zoneForBpm(10_000).cls).toBe('alarm');
    expect(zoneForBpm(0).cls).toBe('calm');
  });
});

describe('hexToRgba', () => {
  it('converts a hex colour to rgba with the requested alpha', () => {
    expect(hexToRgba('#52C27A', 0.2)).toBe('rgba(82,194,122,0.2)');
    expect(hexToRgba('52C27A', 1)).toBe('rgba(82,194,122,1)');
  });

  it('accepts lower case digits', () => {
    expect(hexToRgba('#52c27a', 1)).toBe(hexToRgba('#52C27A', 1));
  });

  it('falls back rather than emitting NaN for malformed input', () => {
    const fallback = hexToRgba(FALLBACK_HEX, 0.5);
    for (const bad of ['', '#', 'red', '#12345', '#GGGGGG', '#52C27A00']) {
      expect(hexToRgba(bad, 0.5)).toBe(fallback);
    }
    expect(fallback).not.toContain('NaN');
  });
});

describe('DEFAULT_COLORS', () => {
  it('has an entry for every zone', () => {
    for (const zone of ZONES) {
      expect(DEFAULT_COLORS[zone.cls]).toBeDefined();
      expect(hexToRgba(DEFAULT_COLORS[zone.cls], 1)).not.toContain('NaN');
    }
  });
});
