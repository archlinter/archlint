# Installation

archlint can be used as a CLI tool or as an ESLint plugin.

## CLI Tool (Recommended)

The easiest way to use archlint is via `npx`. This ensures you're always using the latest version without adding it to your `package.json`.

```bash
npx @archlinter/cli scan
```

### Global Installation

If you want to install archlint globally for use across all projects:

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

After global installation, you can run `archlint` directly:

```bash
archlint scan
```

### Local Installation

Alternatively, you can install it as a dev dependency in your project:

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

### From Source (Rust)

If you prefer to use the native binary directly, you can install it via Cargo:

```bash
cargo install archlint
```

## ESLint Plugin

To get real-time architectural feedback in your IDE, install the ESLint plugin:

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

See the [ESLint Integration](/integrations/eslint) section for configuration details.

## MCP Server

If you're using AI coding assistants like Claude or Cursor, you can install our MCP server:

```bash
npx @archlinter/mcp-server
```

See the [MCP Server](/integrations/mcp-server) section for more information.

## GitHub Action

To prevent architectural regressions in your Pull Requests, use our official GitHub Action:

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

See the [GitHub Actions](/integrations/github-actions) section for more information.
