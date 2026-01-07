---
title: ESLint 集成
description: 使用 @archlinter/eslint-plugin 在编辑器中获取实时架构反馈。支持 Flat Config 和旧版 .eslintrc。
---

# ESLint 集成

`@archlinter/eslint-plugin` 将架构反馈直接带入您的编辑器。

## 安装

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

## 配置

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

### 旧版配置 (ESLint < 9)

```javascript
// .eslintrc.js
module.exports = {
  plugins: ['@archlinter'],
  extends: ['plugin:@archlinter/recommended'],
};
```

## 性能

该插件在后台进程中运行 archlint 分析。在第一次运行时，可能需要几秒钟来构建初始依赖图。由于有缓存，后续运行几乎是瞬时的。

## 规则

该插件将 archlint 探测器映射到 ESLint 规则：

- `@archlinter/no-cycles`
- `@archlinter/no-god-modules`
- `@archlinter/no-dead-code`
- `@archlinter/no-layer-violations`
- ... 以及更多。
