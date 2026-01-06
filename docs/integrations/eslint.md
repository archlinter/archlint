# ESLint Integration

The `@archlinter/eslint-plugin` brings architectural feedback directly into your editor.

## Installation

```bash
npm install -D @archlinter/eslint-plugin
```

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
