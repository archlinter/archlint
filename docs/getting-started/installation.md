# Installation

archlint can be used as a CLI tool or as an ESLint plugin.

## CLI Tool (Recommended)

The easiest way to use archlint is via `npx`. This ensures you're always using the latest version without adding it to your `package.json`.

```bash
npx @archlinter/cli scan
```

Alternatively, you can install it globally or as a dev dependency:

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

:::

### From Source (Rust)

If you prefer to use the native binary directly, you can install it via Cargo:

```bash
cargo install archlint
```

## ESLint Plugin

To get real-time architectural feedback in your IDE, install the ESLint plugin:

```bash
npm install -D @archlinter/eslint-plugin
```

See the [ESLint Integration](/integrations/eslint) section for configuration details.

## MCP Server

If you're using AI coding assistants like Claude or Cursor, you can install our MCP server:

```bash
npx @archlinter/mcp-server
```

See the [MCP Server](/integrations/mcp-server) section for more information.
