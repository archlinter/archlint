---
title: MCP Server
description: "使用模型上下文协议 (MCP) 将 archlint 连接到 Claude 或 Cursor 等 AI 编程助手，实现 AI 驱动的架构重构。"
---

# MCP Server

archlint 提供了一个 MCP (Model Context Protocol) 服务，允许像 Claude 或 Cursor 这样的 AI 编程助手理解并改进您的架构。

## 为什么使用 MCP Server？

- **AI 驱动的重构**：您的 AI 助手可以看到架构坏味道，并建议具体的代码更改来修复它们。
- **上下文知识**：助手可以询问“为什么这是一个 God Module？”并根据实际分析获得详细的回答。
- **自动化修复**：要求助手“修复此文件夹中的所有循环依赖”，它可以使用 archlint 的分析来执行重构。

## 安装

::: code-group

```bash [npm]
npx @archlinter/mcp-server
```

```bash [pnpm]
pnpm dlx @archlinter/mcp-server
```

```bash [yarn]
yarn dlx @archlinter/mcp-server
```

```bash [bun]
bunx @archlinter/mcp-server
```

:::

## 快速添加到 Cursor

如果您使用 [Cursor](https://cursor.com)，只需点击一下即可添加 MCP 服务：

<a href="cursor://anysphere.cursor-deeplink/mcp/install?name=archlint&config=eyJjb21tYW5kIjoibnB4IiwiYXJncyI6WyIteSIsIkBhcmNobGludGVyL21jcC1zZXJ2ZXIiXX0=" class="add-to-cursor-btn">
  <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <path d="M11.503.131 1.891 5.678a.84.84 0 0 0-.42.726v11.188c0 .3.162.575.42.724l9.609 5.55a1 1 0 0 0 .998 0l9.61-5.55a.84.84 0 0 0 .42-.724V6.404a.84.84 0 0 0-.42-.726L12.497.131a1.01 1.01 0 0 0-.996 0M2.657 6.338h18.55c.263 0 .43.287.297.515L12.23 22.918c-.062.107-.229.064-.229-.06V12.335a.59.59 0 0 0-.295-.51l-9.11-5.257c-.109-.063-.064-.23.061-.23" fill="currentColor"/>
  </svg>
  Add to Cursor
</a>

## 手动配置 (Cursor/Claude Desktop)

将以下内容添加到您的 MCP 设置中：

```json
{
  "mcpServers": {
    "archlint": {
      "command": "npx",
      "args": ["-y", "@archlinter/mcp-server"]
    }
  }
}
```

## 可用工具

MCP 服务向 AI 公开了几个工具：

- `archlint_scan`：执行完整扫描并返回坏味道列表。
- `archlint_explain`：解释特定的坏味道并提供重构建议。
- `archlint_stats`：为项目提供高层级的架构指标。
