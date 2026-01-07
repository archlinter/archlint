---
title: Integração com ESLint
description: Receba feedback arquitetural em tempo real no seu editor usando @archlinter/eslint-plugin. Suporta Flat Config e .eslintrc legado.
---

# Integração com ESLint

O `@archlinter/eslint-plugin` traz feedback arquitetural diretamente para o seu editor.

## Instalação

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

## Configuração

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

### Configuração Legada (ESLint < 9)

```javascript
// .eslintrc.js
module.exports = {
  plugins: ['@archlinter'],
  extends: ['plugin:@archlinter/recommended'],
};
```

## Performance

O plugin executa a análise do archlint em um processo em segundo plano. Na primeira execução, pode levar alguns segundos para construir o grafo de dependências inicial. As execuções subsequentes são quase instantâneas devido ao cache.

## Regras

O plugin mapeia os detectores do archlint para regras do ESLint:

- `@archlinter/no-cycles`
- `@archlinter/no-god-modules`
- `@archlinter/no-dead-code`
- `@archlinter/no-layer-violations`
- ... e mais.
