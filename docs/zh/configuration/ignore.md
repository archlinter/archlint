# 忽略模式

archlint 提供了多种从分析中排除文件或目录的方法。

## 全局忽略

`archlint.yaml` 中的 `ignore` 部分指定了应被所有探测器完全跳过的文件。

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## .gitignore 支持

默认情况下，archlint 会自动遵循您的 `.gitignore` 文件。您不需要在 `archlint.yaml` 中重复这些模式。

## 探测器特定忽略

某些探测器在 `thresholds` 部分有自己的 `exclude_patterns`。如果您希望一个文件被大多数探测器分析，但被特定的探测器跳过（例如，从循环依赖检测中排除测试文件），这将非常有用。

```yaml
thresholds:
  cycles:
    exclude_patterns:
      - '**/*.test.ts'
      - '**/*.spec.ts'
```

## 行内忽略

（即将推出）我们正在努力支持像 `// archlint-disable` 这样的行内注释，以便直接在源代码中忽略特定的行或文件。
