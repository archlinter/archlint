# 发布流程

本文档描述了 archlint 的发布流程。

## 概述

archlint 使用 **semantic-release** 来自动化整个发布工作流。版本号是根据遵循 Conventional Commits 格式的提交消息计算得出的。

## 提交消息格式

所有提交**必须**遵循 Conventional Commits 格式。这由 CI 中的 commitlint 强制执行。

### 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 类型

| 类型       | 描述       | 版本提升          |
| ---------- | ---------- | ----------------- |
| `feat`     | 新特性     | **Minor** (0.x.0) |
| `fix`      | 错误修复   | **Patch** (0.0.x) |
| `perf`     | 性能优化   | **Patch** (0.0.x) |
| `refactor` | 代码重构   | 无                |
| `docs`     | 文档       | 无                |
| `test`     | 测试       | 无                |
| `chore`    | 维护       | 无                |
| `ci`       | CI/CD 更改 | 无                |
| `build`    | 构建系统   | 无                |

### 重大更改 (Breaking Changes)

在类型后添加 `!` 或在正文末尾添加 `BREAKING CHANGE:` 以触发 **Major** 版本提升：

```bash
# Major 版本提升 (1.0.0)
git commit -m "feat!: change API signature"

# 或者
git commit -m "feat: new feature

BREAKING CHANGE: This changes the public API"
```

## 发布流程

### 1. 开发

在特性分支中开发功能，并将其合并到 `main` 分支。

### 预发布分支

`.releaserc.json` 文件包含 `beta` 和 `alpha` 通道的静态分支配置。但是，**预发布分支由 CI 在发布工作流期间动态配置**。该工作流会根据所选通道和当前分支名称自动创建分支配置，因此在实际发布过程中不会使用 `.releaserc.json` 中的静态条目。

### 2. 触发发布

当您准备好发布时，手动触发 Release 工作流：

1. 转到 **Actions** -> **Release** 工作流。
2. 点击 **Run workflow**。
3. (可选) 将 `dry_run` 设置为 `true`，以查看在不实际发布的情况下会发生什么。

### 3. 自动步骤

工作流将：

1. **计算版本**：`semantic-release` 分析自上次发布以来的提交。
2. **更新文件**：自动更新 `Cargo.toml`、`package.json` 和 `CHANGELOG.md`。
3. **提交和打标签**：为发布创建一个新的提交和 Git 标签（tag）。
4. **触发 CI**：标签推送会触发 CI 工作流，构建所有二进制文件。
5. **发布到 npm**：CI 将所有包发布到 npm 仓库（仅在有标签时）。
6. **附加二进制文件**：CI 将独立运行的二进制文件上传到 GitHub Release。

## 版本号

所有包共享相同的版本（统一版本管理）：

- `@archlinter/cli@0.2.0`
- `@archlinter/cli-darwin-arm64@0.2.0`
- `@archlinter/cli-linux-x64@0.2.0`
- 等等。

## 检查发布状态

### 查看工作流状态

https://github.com/archlinter/archlint/actions

### 验证 npm 发布

```bash
npm view @archlinter/cli
```

### 测试安装

```bash
npx @archlinter/cli@latest --version
```

## 故障排除

### 提交被 commitlint 拒绝

**修复方法**：遵循常规提交格式：

```bash
git commit --amend -m "feat: correct commit message"
```

### 发布工作流失败

检查：

1. NPM_TOKEN secret 是否配置？
2. GH_PAT secret 是否配置？
3. CI 构建是否失败？

## 参考

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [semantic-release](https://github.com/semantic-release/semantic-release)
