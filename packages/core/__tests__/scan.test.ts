import { describe, it, expect } from 'vitest';
import { scan, scanSync, getDetectors, loadConfig } from '../index';
import { fixtures } from './fixtures';

describe('scan', () => {
  it('detects cycles in test project', async () => {
    const result = await scan(fixtures.cycles);

    expect(result.summary.filesAnalyzed).toBeGreaterThan(0);
    expect(result.smells.some((s) => s.smell.smellType.includes('Cyclic'))).toBe(true);
  });

  it('respects detector filter', async () => {
    const result = await scan(fixtures.cycles, {
      detectors: ['cycles'],
    });

    // In our implementation, smellType is the debug string of the enum
    expect(result.smells.every((s) => s.smell.smellType.includes('Cyclic'))).toBe(true);
  });
});

describe('scanSync', () => {
  it('works synchronously', () => {
    const result = scanSync(fixtures.cycles);
    expect(result.summary.filesAnalyzed).toBeGreaterThan(0);
  });
});

describe('getDetectors', () => {
  it('returns all detectors', () => {
    const detectors = getDetectors();

    expect(detectors.length).toBeGreaterThan(20);
    expect(detectors.find((d) => d.id === 'cycles')).toBeDefined();
  });
});

describe('loadConfig', () => {
  it('loads default config', () => {
    const config = loadConfig();
    expect(config.ignore).toBeDefined();
  });
});
