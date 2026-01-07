# 循環依存 (Cyclic Dependencies)

**ID:** `cycles` | **重要度:** 致命的 (デフォルト)

循環依存は、2つ以上のモジュールが直接的または間接的に互いに依存している場合に発生します。

## なぜこれが「不吉な臭い（スメル）」なのか

- **密結合**: モジュールが切り離せなくなり、個別に再利用することが困難になります。
- **初期化の問題**: バンドラーによる処理が適切でない場合、実行時に「undefined」なインポートが発生する可能性があります。
- **テストの困難さ**: サイクル全体を取り込まずに、1つのモジュールをモックしたり隔離したりすることが難しくなります。
- **認知負荷**: 開発者がデータや制御の流れを理解するのが難しくなります。

## 例 (Examples)

### Bad

```typescript
// orders.ts
import { processPayment } from './payments';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { createOrder } from './orders';
export const processPayment = () => {
  /* ... */
};
```

### Good

共有ロジックを第3のモジュールに抽出します。

```typescript
// types.ts
export interface Order {
  /* ... */
}

// orders.ts
import { Order } from './types';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { Order } from './types';
export const processPayment = (order: Order) => {
  /* ... */
};
```

## 設定 (Configuration)

```yaml
thresholds:
  cycles:
    exclude_patterns: ['**/*.test.ts']
```

## 修正方法

1. **共有ロジックの抽出**: 共通部分を、既存のモジュールの両方が依存する新しいモジュールに移動してください。
2. **依存性の注入 (Dependency Injection)**: 依存関係をインポートするのではなく、引数として渡してください。
3. **イベントの使用**: イベントバスやコールバックを使用して、モジュール間の結合を解除してください。
