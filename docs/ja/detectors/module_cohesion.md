---
title: 分散したモジュール
description: "内部要素が適切に接続されていないモジュールを特定し、凝集性のある目的の欠如と「一緒に変更されるものは一緒に保つべき」という原則の違反を示します。"
---

# 分散したモジュール

**ID:** `module_cohesion` | **重要度:** Medium (default)

内部要素（関数、クラス）がうまく接続されていない「モジュール」（通常はファイルまたは論理グループ）を特定します。これは、モジュールに一貫した目的がなく、無関係なコードの寄せ集めである可能性が高いことを示しています。

## なぜこれが「不吉な臭い」なのか

モジュールは凝集しているべきであり、「一緒に変更されるものは一緒にあるべき」という原則に従います。モジュールの内部パーツが互いに相互作用していない場合、それは本物のモジュールではなく、単なるランダムな保管場所として使用されているフォルダやファイルです。これによりコードを見つけにくくなり、認知負荷が増加します。

## 例

### Bad

共通のロジックやデータを共有しない無関係なヘルパー関数を含むファイル。

```typescript
// misc-utils.ts
export const formatCurrency = (val: number) => { ... };
export const validateEmail = (email: string) => { ... };
export const parseJwt = (token: string) => { ... };
// これら3つの関数は共通の状態やロジックを共有していません。
```

### Good

無関係な関数を特定の凝集したモジュールにグループ化します。

```typescript
// currency-utils.ts
export const formatCurrency = (val: number) => { ... };

// validation-utils.ts
export const validateEmail = (email: string) => { ... };
```

## 設定

```yaml
rules:
  module_cohesion:
    severity: medium
    min_exports: 5
    max_components: 2
```

## 修正方法

モジュールの目的を再評価してください。コードをより凝集度の高いモジュールにグループ化するか、関連のないパーツを実際に使用されている場所に移動してください。
