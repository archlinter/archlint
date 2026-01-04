import { defineConfig } from 'tsup';

export default defineConfig([
  // ESM build
  {
    entry: ['src/index.ts'],
    format: ['esm'],
    dts: true,
    clean: true,
    splitting: false,
    sourcemap: true,
    external: ['@archlinter/core', 'eslint'],
  },
  // CJS build with ESLint 8 compatibility footer
  {
    entry: ['src/index.ts'],
    format: ['cjs'],
    dts: false,
    clean: false,
    splitting: false,
    sourcemap: true,
    external: ['@archlinter/core', 'eslint'],
    footer: {
      js: 'if (module.exports.default) { Object.assign(module.exports.default, module.exports); module.exports = module.exports.default; }',
    },
  },
]);
