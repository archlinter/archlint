---
title: 忽略文件
description: '了解如何使用全局忽略、gitignore、特定规则排除和内联注释从 archlint 分析中排除文件或目录。'
---

# 忽略文件

archlint 提供了几种从分析中排除文件或目录的方法。

## 全局忽略

`.archlint.yaml` 根目录下的 `ignore` 部分用于将文件排除在分析之外。被忽略的文件仍会被解析，因此经过它们的导入依然可以正确解析，但不会为它们报告任何异味，它们也不会计入任何检测器阈值（例如 `min_dependents` 或 `max_files_per_package`）。

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

模式会与相对于项目根目录的路径进行匹配：

- 普通路径 —— `legacy`、`src/legacy`、`generated.ts` —— 匹配该条目本身及其下的所有内容，任意深度。
- 包含 `*`、`?` 或 `[` 的模式会被当作 glob 使用。注意 `*` 也会匹配 `/`，因此 `src/*.ts` 会覆盖 `src/a/b/c.ts`。
- 无法使用的模式会以警告形式报告并被跳过，其余模式仍然生效。

上述规则的唯一例外是循环依赖：循环是关于一组文件的事实，因此同时经过生效文件和被忽略文件的循环仍会被报告，并列出其中被忽略的成员。所有成员都被忽略的循环则不会被报告。

设置 `ignore` 会**替换**内置默认值（`**/*.test.ts`、`**/__tests__/**` 以及其他测试文件模式），而不是在其基础上追加。如果仍希望排除测试文件，请重新列出需要的模式。

## .gitignore 支持

默认情况下，archlint 会自动遵循您的 `.gitignore` 文件。您不需要在 `.archlint.yaml` 中重复这些模式。如果您想禁用此行为，请设置 `git: { enabled: false }`。

## 按规则忽略

您可以使用 `rules` 部分内的 `exclude` 字段从特定检测器中排除文件。如果您希望一个文件被大多数检测器分析但被某个特定检测器跳过，这很有用。

```yaml
rules:
  cycles:
    exclude:
      - '**/generated/**'
      - '**/*.entity.ts'
```

## 路径覆盖 (Overrides)

对于更复杂的逻辑（例如，更改设置或为特定目录禁用多个规则），请使用 `overrides` 部分：

```yaml
overrides:
  - files: ['**/tests/**', '**/mocks/**']
    rules:
      cyclomatic_complexity: off
      god_module: off
      large_file: medium
```

## 内联忽略 (Inline Ignore)

您可以使用特殊的注释直接在源代码中忽略特定的架构问题。这对于在例外情况下抑制警告非常有用。

### 用法：

所有模式均支持单行注释（`// archlint-...`）和块注释（`/* archlint-... */`）语法。

1. **整个文件**：在文件顶部添加 `// archlint-disable`。
2. **当前行**：在行尾或上一行添加 `// archlint-disable-line`。
3. **下一行**：在有问题的行之前使用 `// archlint-disable-next-line`。
4. **代码块**：使用 `// archlint-disable` 和 `// archlint-enable` 来包裹一段代码。

### 示例：

```typescript
// archlint-disable * - 整个文件使用遗留模式
// 忽略整个文件的所有规则

// prettier-ignore
// archlint-disable-next-line long-params - 此遗留函数需要许多参数
function processTransaction(id: string, amount: number, currency: string, date: Date, recipient: string, note: string) {
  // 长参数检测将仅针对此行被忽略
}

import { internal } from './private'; // archlint-disable-line layer_violation - 迁移的临时排除

/* archlint-disable cyclomatic_complexity */
function legacyCode() {
  // 此代码块将被忽略
}
/* archlint-enable cyclomatic_complexity */
```

您可以指定多个以逗号分隔的规则，或使用 `*` 忽略所有规则。
