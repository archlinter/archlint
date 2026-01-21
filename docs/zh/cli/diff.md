---
title: diff
description: "将当前代码库与基线进行比较，以检测新的架构回归和恶化的代码异味，支持棘轮（Ratchet）理念。"
---

# archlint diff

`diff` 命令是实现“棘轮（Ratchet）”持续改进机制的核心功能。它将当前的代码库与之前保存的快照或其他 git 分支/提交进行比较。

## 用法

```bash
# 与快照文件进行比较
archlint diff <baseline.json> [options]

# 与 git 引用进行比较
archlint diff <git-ref> [options]
```

## 工作原理

archlint 不仅仅是计算问题数量。它对架构问题（smells）进行**语义差分（semantic diff）**：

1. **新架构问题**：当前存在但基线中不存在的问题（例如，一个新的循环依赖）。
2. **恶化的问题**：现有的问题变得更严重（例如，一个循环依赖从 3 个文件增加到 5 个文件）。
3. **已修复的问题**：基线中存在但现在已消失的问题。

## 选项

| 选项                   | 默认值  | 描述                                              |
| ---------------------- | ------- | ------------------------------------------------- |
| `--fail-on <severity>` | `low`   | 如果发现此严重程度或更高的退化，则以退出码 1 退出 |
| `--explain`            | `false` | 为每个退化提供详细解释                            |

## 配置

您可以在 `.archlint.yaml` 文件中微调 diff 引擎：

```yaml
diff:
  metric_threshold_percent: 20 # 仅当指标恶化 >20% 时报告为退化
  line_tolerance: 50 # 模糊匹配期间忽略 50 行以内的偏移
```

有关详细信息，请参阅[配置指南](/zh/configuration/index#diff-配置)。

## 示例

### 在 CI 中对照 main 分支进行检查

```bash
archlint diff origin/main --fail-on low --explain
```

### 对照本地基线进行检查

```bash
archlint diff .archlint-baseline.json
```
