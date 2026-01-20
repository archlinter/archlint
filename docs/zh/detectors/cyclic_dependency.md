# 循环依赖 (Cyclic Dependencies)

**ID:** `cycles` | **严重程度:** 紧急 (默认)

当两个或多个模块直接或间接地相互依赖时，就会发生循环依赖。这就是软件工程中经典的“先有鸡还是先有蛋”的问题。

## 为什么这是一种坏味道

- **无法拆分的耦合**：你不能简单地把一个模块拿出来单独使用，它会强迫你把整个依赖“家族”都带上。
- **初始化陷阱**：取决于你的打包工具，你可能会在运行时遇到 `undefined` 导入，因为循环依赖导致模块无法按时完成初始化。
- **测试噩梦**：想要在不引起整个循环崩溃的情况下模拟（mock）其中一部分，简直就像在玩随时会倒塌的积木。
- **认知负担**：试图追踪循环依赖中的数据流，就像在读一本每一页都会让你跳回开头的“死循环”小说。

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
