---
title: ESLintとの統合
description: "@archlinter/eslint-pluginを使用して、エディタでリアルタイムのアーキテクチャフィードバックを取得します。Flat Configおよびレガシーな.eslintrcをサポートしています。"
---
---

# ESLintとの統合

`@archlinter/eslint-plugin` は、アーキテクチャに関するフィードバックをエディタに直接提供します。

## インストール

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

## 設定

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

### レガシー設定 (ESLint < 9)

```javascript
// .eslintrc.js
module.exports = {
  plugins: ['@archlinter'],
  extends: ['plugin:@archlinter/recommended'],
};
```

## パフォーマンス

このプラグインは、バックグラウンドプロセスでarchlintの分析を実行します。初回実行時は、初期依存関係グラフの構築に数秒かかる場合があります。2回目以降の実行は、キャッシュによりほぼ瞬時に完了します。

## ルール

このプラグインは、archlintの検出器をESLintルールにマッピングします：

- `@archlinter/no-cycles`
- `@archlinter/no-god-modules`
- `@archlinter/no-dead-code`
- `@archlinter/no-layer-violations`
- ... その他多数。
