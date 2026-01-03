import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  clean: true,
  splitting: false,
  sourcemap: true,
  external: ['@archlinter/core', 'eslint'],
  footer: {
    // Make default export the main export for CJS (ESLint 8 compatibility)
    js: 'if (module.exports.default) { Object.assign(module.exports.default, module.exports); module.exports = module.exports.default; }',
  },
});
