# MCP Server

archlint provides an MCP (Model Context Protocol) server, allowing AI coding assistants like Claude or Cursor to understand and improve your architecture.

## Why use the MCP Server?

- **AI-Powered Refactoring**: Your AI assistant can see the architectural smells and suggest specific code changes to fix them.
- **Contextual Knowledge**: The assistant can ask "Why is this a God Module?" and get a detailed response based on the actual analysis.
- **Automated Fixes**: Ask the assistant to "Fix all circular dependencies in this folder," and it can use archlint's analysis to perform the refactoring.

## Installation

You can run the MCP server using `npx`:

```bash
npx @archlinter/mcp-server
```

## Configuration (Cursor/Claude Desktop)

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
