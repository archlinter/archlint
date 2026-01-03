# @archlinter/eslint-plugin

ESLint plugin for architectural smell detection using [archlint](https://github.com/archlinter/archlint).

## Installation

```bash
npm install --save-dev @archlinter/eslint-plugin eslint
```

## Usage

### Flat Config (ESLint 9+)

```javascript
// eslint.config.js
import archlint from '@archlinter/eslint-plugin';

export default [
  archlint.configs['flat/recommended'],
  {
    rules: {
      '@archlinter/no-cycles': 'error',
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

## Rules

- `@archlinter/no-cycles`: Disallow cyclic dependencies.
- `@archlinter/no-god-modules`: Disallow overly complex modules.
- `@archlinter/no-dead-code`: Disallow unused exports.
- `@archlinter/no-layer-violations`: Enforce architectural layers.
- ...and more.

## Performance

The plugin runs archlint analysis in the background and caches results per project root. On the first run, it may show an informational message while the analysis is in progress.

## License

MIT
