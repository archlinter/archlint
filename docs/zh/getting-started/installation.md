---
title: 安装
description: "通过 npx 或 npm 将 archlint 安装为 CLI 工具，或将其用作 ESLint 插件以在编辑器中获得实时反馈。"
---

# 安装

archlint 可以作为 CLI 工具或 ESLint 插件使用。

## CLI 工具（推荐）

使用 archlint 最简单的方法是通过 `npx`。这可以确保您始终使用最新版本，而无需将其添加到 `package.json` 中。

```bash
npx @archlinter/cli scan
```

### 全局安装

如果您想全局安装 archlint 以便在所有项目中使用：

::: code-group

```bash [npm]
npm install -g @archlinter/cli
```

```bash [pnpm]
pnpm add -g @archlinter/cli
```

```bash [yarn]
yarn global add @archlinter/cli
```

```bash [bun]
bun add -g @archlinter/cli
```

```bash [deno]
deno install -g npm:@archlinter/cli
```

:::

全局安装后，您可以直接运行 `archlint`：

```bash
archlint scan
```

### 本地安装

或者，您可以将其作为开发依赖安装在项目中：

::: code-group

```bash [npm]
npm install -D @archlinter/cli
```

```bash [pnpm]
pnpm add -D @archlinter/cli
```

```bash [yarn]
yarn add -D @archlinter/cli
```

```bash [bun]
bun add -D @archlinter/cli
```

```bash [deno]
deno install npm:@archlinter/cli
```

:::

### 从源码安装 (Rust)

如果您更愿意直接使用原生二进制文件，可以通过 Cargo 安装：

```bash
cargo install archlint
```

## ESLint 插件

要在 IDE 中获取实时架构反馈，请安装 ESLint 插件：

::: code-group

```bash [npm]
npm install -D @archlinter/eslint-plugin
```

```bash [pnpm]
pnpm add -D @archlinter/eslint-plugin
```

```bash [yarn]
yarn add -D @archlinter/eslint-plugin
```

```bash [bun]
bun add -D @archlinter/eslint-plugin
```

```bash [deno]
deno install npm:@archlinter/eslint-plugin
```

:::

有关配置详细信息，请参阅 [ESLint 集成](/zh/integrations/eslint) 章节。

## MCP 服务器

如果您正在使用 Claude 或 Cursor 等 AI 编程助手，可以安装我们的 MCP 服务器：

```bash
npx @archlinter/mcp-server
```

有关更多信息，请参阅 [MCP 服务器](/zh/integrations/mcp-server) 章节。

## GitHub Action

要防止 Pull Request 中的架构退化，请使用我们的官方 GitHub Action：

<div v-pre>

```yaml
- name: archlint
  uses: archlinter/action@v1
  with:
    baseline: origin/${{ github.base_ref }}
    fail-on: medium
    github-token: ${{ github.token }}
```

</div>

有关更多信息，请参阅 [GitHub Actions](/zh/integrations/github-actions) 章节。
