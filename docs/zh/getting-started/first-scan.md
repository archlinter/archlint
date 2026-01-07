# 第一次扫描

安装完成后，运行您的第一次扫描非常简单。

## 运行基础扫描

导航到您的项目根目录并运行：

```bash
npx @archlinter/cli scan
```

默认情况下，archlint 将：

1. 扫描当前目录中的所有 TypeScript 和 JavaScript 文件。
2. 遵循您的 `.gitignore` 文件。
3. 为所有 28+ 个检测器使用默认规则。
4. 输出检测到的异味的彩色表格摘要。

## 保存快照 (Snapshot)

要使用“棘轮”方法，您首先需要捕获架构的当前状态：

```bash
npx @archlinter/cli snapshot -o .archlint-baseline.json
```

此文件代表您的架构基线。您应该将其提交到您的仓库中。

## 检查退化 (Regressions)

现在，在开发过程中，您可以检查您的更改是否引入了任何新的架构问题：

```bash
npx @archlinter/cli diff .archlint-baseline.json
```

在 CI 环境中，您通常会与主分支进行比较：

```bash
npx @archlinter/cli diff origin/main --fail-on medium
```

## 下一步是什么？

- [了解所有检测器](/zh/detectors/)
- [配置 .archlint.yaml](/zh/configuration/)
- [集成到 CI/CD](/zh/integrations/github-actions)
