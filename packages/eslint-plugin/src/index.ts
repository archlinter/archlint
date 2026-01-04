import type { ESLint } from 'eslint';
import { rules } from './rules';
import { recommended, strict, flatRecommended, flatStrict } from './configs';

const plugin: ESLint.Plugin = {
  meta: {
    name: '@archlinter/eslint-plugin',
    version: '0.6.0-alpha.1',
  },
  rules,
  configs: {
    // Legacy configs (ESLint 8)
    // @ts-expect-error - Legacy config format compatibility
    recommended,
    // @ts-expect-error - Legacy config format compatibility
    strict,
    // Flat configs (ESLint 9+)
    'flat/recommended': flatRecommended,
    'flat/strict': flatStrict,
  },
};

// Dual export for CJS and ESM compatibility
export default plugin;
export { rules } from './rules';
export { recommended, strict, flatRecommended, flatStrict } from './configs';

// Export test utilities
export { notifyFileChanged, clearAllCaches } from './utils/cache';
