import { describe, it, expect } from 'vitest';
import plugin from '../index';

describe('eslint-plugin', () => {
  it('should export rules', () => {
    expect(plugin.rules).toBeDefined();
    expect(plugin.rules['no-cycles']).toBeDefined();
    expect(plugin.rules['no-god-modules']).toBeDefined();
  });

  it('should export configs', () => {
    expect(plugin.configs).toBeDefined();
    expect(plugin.configs.recommended).toBeDefined();
    expect(plugin.configs.strict).toBeDefined();
    expect(plugin.configs['flat/recommended']).toBeDefined();
    expect(plugin.configs['flat/strict']).toBeDefined();
  });
});
