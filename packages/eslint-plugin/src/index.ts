import type { ESLint } from 'eslint';
import { rules } from './rules';
import { recommended, strict, flatRecommended, flatStrict } from './configs';

const plugin: ESLint.Plugin = {
  meta: {
    name: '@archlinter/eslint-plugin',
    version: '0.5.0',
  },
  rules,
  configs: {
    // Legacy configs (ESLint 8)
    recommended,
    strict,
    // Flat configs (ESLint 9+)
    'flat/recommended': flatRecommended,
    'flat/strict': flatStrict,
  },
};

// Dual export for CJS and ESM compatibility
export default plugin;
export { rules, recommended, strict, flatRecommended, flatStrict };
