---
title: Integración con ESLint
description: Obtén retroalimentación arquitectónica en tiempo real en tu editor usando @archlinter/eslint-plugin. Soporta Flat Config y el formato heredado .eslintrc.
---

# Integración con ESLint

El paquete `@archlinter/eslint-plugin` lleva la retroalimentación arquitectónica directamente a tu editor.

## Instalación

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

## Configuración

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

### Configuración Heredada (Legacy) (ESLint < 9)

```javascript
// .eslintrc.js
module.exports = {
  plugins: ['@archlinter'],
  extends: ['plugin:@archlinter/recommended'],
};
```

## Rendimiento

El plugin ejecuta el análisis de archlint en un proceso en segundo plano. En la primera ejecución, puede tardar unos segundos en construir el gráfico de dependencias inicial. Las ejecuciones posteriores son casi instantáneas gracias al almacenamiento en caché.

## Reglas

El plugin mapea los detectores de archlint a reglas de ESLint:

- `@archlinter/no-cycles`
- `@archlinter/no-god-modules`
- `@archlinter/no-dead-code`
- `@archlinter/no-layer-violations`
- ... y más.
