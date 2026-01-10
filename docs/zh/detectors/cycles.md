# 循环依赖 (Cyclic Dependencies)

**ID:** `cycles` | **严重程度:** 紧急 (默认)

当两个或多个模块直接或间接地相互依赖时，就会发生循环依赖。

## 为什么这是一种坏味道

- **紧密耦合**: 模块不可分割，难以独立重用。
- **初始化问题**: 如果打包工具处理不当，可能会导致运行时的"undefined"导入。
- **测试困难**: 难以在不引入整个循环的情况下模拟或隔离单个模块。
- **认知负荷**: 开发人员更难理解数据和控制流。

## 示例

### 坏习惯

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

### 好习惯

将共享逻辑提取到第三个模块中。

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

## 配置

```yaml
rules:
  cycles:
    severity: high
    exclude: ['**/*.test.ts']
```

## 如何修复

1. **提取共享逻辑**: 将公共部分移动到两个现有模块都依赖的新模块中。
2. **依赖注入 (Dependency Injection)**: 将依赖项作为参数传递，而不是直接导入它们。
3. **使用事件**: 使用事件总线或回调来解耦模块。

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-cycles': 'error',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。
