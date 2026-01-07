---
title: ESLint Integration
description: Get real-time architectural feedback in your editor using @archlinter/eslint-plugin. Supports Flat Config and legacy .eslintrc.
---

# ESLint Integration

The `@archlinter/eslint-plugin` brings architectural feedback directly into your editor.

## Installation

::: code-group

```bash [npm]
npm install -D @archlinter/eslint-plugin
```

```bash [pnpm]
pnpm add -D @archlinter/eslint-plugin
```

```bash [yarn]
yarn add -D @archlinter/eslint-plugin
```

```bash [bun]
bun add -D @archlinter/eslint-plugin
```

```bash [deno]
deno install npm:@archlinter/eslint-plugin
```

:::

## Configuration

### Flat Config (ESLint 9+)

```javascript
// eslint.config.js
import archlint from '@archlinter/eslint-plugin';

export default [
  archlint.configs['flat/recommended'],
  {
    rules: {
      '@archlinter/no-cycles': 'error',
      '@archlinter/no-god-modules': 'warn',
    },
  },
];
```

### Legacy Config (ESLint < 9)

```javascript
// .eslintrc.js
module.exports = {
  plugins: ['@archlinter'],
  extends: ['plugin:@archlinter/recommended'],
};
```

## Performance

The plugin runs archlint analysis in a background process. On the first run, it may take a few seconds to build the initial dependency graph. Subsequent runs are near-instant due to caching.

## Rules

The plugin maps archlint detectors to ESLint rules:

- `@archlinter/no-cycles`
- `@archlinter/no-god-modules`
- `@archlinter/no-dead-code`
- `@archlinter/no-layer-violations`
- ... and more.
