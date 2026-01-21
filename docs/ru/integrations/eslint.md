---
title: Интеграция с ESLint
description: "Получайте архитектурную обратную связь в реальном времени прямо в редакторе с помощью @archlinter/eslint-plugin. Поддерживает Flat Config и классический .eslintrc."
---

# Интеграция с ESLint

Плагин `@archlinter/eslint-plugin` переносит архитектурную обратную связь прямо в ваш редактор.

## Установка

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

## Конфигурация

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

## Производительность

Плагин запускает анализ archlint в фоновом процессе. При первом запуске построение начального графа зависимостей может занять несколько секунд. Последующие запуски происходят почти мгновенно благодаря кэшированию.

## Правила

Плагин сопоставляет детекторы archlint с правилами ESLint:

- `@archlinter/no-cycles`
- `@archlinter/no-god-modules`
- `@archlinter/no-dead-code`
- `@archlinter/no-layer-violations`
- ... и другие.
