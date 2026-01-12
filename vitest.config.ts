import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['**/__tests__/**/*.test.ts'],
    passWithNoTests: true,
    // Use forks instead of threads for native modules stability,
    // especially on Windows where it's prone to C0000005 crashes on exit
    pool: process.platform === 'win32' ? 'forks' : 'threads',
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['packages/*/src/**/*.ts'],
    },
  },
});
