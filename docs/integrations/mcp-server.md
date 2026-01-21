---
title: MCP Server
description: "Connect archlint to AI coding assistants like Claude or Cursor using the Model Context Protocol (MCP) for AI-powered architectural refactoring."
---

# MCP Server

archlint provides an MCP (Model Context Protocol) server, allowing AI coding assistants like Claude or Cursor to understand and improve your architecture.

## Why use the MCP Server?

- **AI-Powered Refactoring**: Your AI assistant can see the architectural smells and suggest specific code changes to fix them.
- **Contextual Knowledge**: The assistant can ask "Why is this a God Module?" and get a detailed response based on the actual analysis.
- **Automated Fixes**: Ask the assistant to "Fix all circular dependencies in this folder," and it can use archlint's analysis to perform the refactoring.

## Installation

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

## Quick Add to Cursor

If you use [Cursor](https://cursor.com), you can add the MCP server with a single click:

<a href="cursor://anysphere.cursor-deeplink/mcp/install?name=archlint&config=eyJjb21tYW5kIjoibnB4IiwiYXJncyI6WyIteSIsIkBhcmNobGludGVyL21jcC1zZXJ2ZXIiXX0=" class="add-to-cursor-btn">
  <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <path d="M11.503.131 1.891 5.678a.84.84 0 0 0-.42.726v11.188c0 .3.162.575.42.724l9.609 5.55a1 1 0 0 0 .998 0l9.61-5.55a.84.84 0 0 0 .42-.724V6.404a.84.84 0 0 0-.42-.726L12.497.131a1.01 1.01 0 0 0-.996 0M2.657 6.338h18.55c.263 0 .43.287.297.515L12.23 22.918c-.062.107-.229.064-.229-.06V12.335a.59.59 0 0 0-.295-.51l-9.11-5.257c-.109-.063-.064-.23.061-.23" fill="currentColor"/>
  </svg>
  Add to Cursor
</a>

## Manual Configuration (Cursor/Claude Desktop)

Add the following to your MCP settings:

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

## Available Tools

The MCP server exposes several tools to the AI:

- `archlint_scan`: Performs a full scan and returns a list of smells.
- `archlint_explain`: Explains a specific smell and provides refactoring advice.
- `archlint_stats`: Provides high-level architectural metrics for the project.
