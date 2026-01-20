# 死代码 (Dead Code)

**ID:** `dead_code` | **严重程度:** 低 (默认)

“死代码”正如其名：那些在你的代码库中“活着”，但实际上却没有任何作用的函数、类或变量，因为根本没人在用它们。

## 为什么这是一种坏味道

- **浪费精力**：开发人员不应该花时间去重构或理解那些根本不会运行的代码。
- **虚假的复杂性**：它会让你的模块 API 看起来比实际更庞大、更吓人。
- **代码“幽灵”**：在调试过程中，它可能会让你发出“我以为我们已经删掉这个了”的感叹，造成不必要地困惑。

## 示例

### 坏习惯

```typescript
// utils.ts
export const usedHelper = () => { ... };
export const unusedHelper = () => { ... }; // 被报告为死代码

// main.ts
import { usedHelper } from './utils';
```

## 如何修复

1. **删除它**: 如果确实未使用，最好的操作是将其移除。
2. **标记为入口点 (Entry Point)**: 如果它是公共 API 或动态导入的一部分，请将其添加到配置中的 `entry_points`。

## 配置

```yaml
# 规则特定选项
rules:
  dead_code:
    exclude:
      - '**/tests/**'
      - '**/temp/**'

# 全局选项 (根级别)
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

### 选项

#### 规则选项 (`rules.dead_code`)

- `exclude`: 检测死代码时要忽略的 glob 模式列表。匹配这些模式的文件在进行入站依赖分析时将被视为不存在。

#### 全局选项 (根级别)

- `entry_points`: 永远不会被报告为死代码的全局入口点。

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-code': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。
