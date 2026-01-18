# 死代码 (Dead Code)

**ID:** `dead_code` | **严重程度:** 低 (默认)

死代码是指在项目中未被任何地方导入或使用的导出函数、类或变量。

## 为什么这是一种坏味道

- **维护负担**: 开发人员可能会花时间更新或重构根本没被使用的代码。
- **包大小**: 增加了最终应用程序的大小（尽管许多打包工具会进行摇树优化 Tree-shaking）。
- **混淆**: 使模块的 API 看起来比实际更大、更复杂。

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
