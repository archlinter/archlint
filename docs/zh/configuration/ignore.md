# 忽略文件

archlint 提供了几种从分析中排除文件或目录的方法。

## 全局忽略

`.archlint.yaml` 根目录下的 `ignore` 部分指定了所有检测器都应完全跳过的文件。

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## .gitignore 支持

默认情况下，archlint 会自动遵循您的 `.gitignore` 文件。您不需要在 `.archlint.yaml` 中重复这些模式。如果您想禁用此行为，请设置 `enable_git: false`。

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
      complexity: off
      god_module: off
      large_file: warn
```

## 内联忽略

（开发中）我们正在支持类似于 `// archlint-disable` 的注释，以便直接在代码中忽略特定的行或文件。
