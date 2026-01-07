# インストール

archlint は、CLI ツールまたは ESLint プラグインとして使用できます。

## CLI ツール (推奨)

archlint を使用する最も簡単な方法は、`npx` を介することです。これにより、`package.json` に追加することなく、常に最新バージョンを使用できます。

```bash
npx @archlinter/cli scan
```

### グローバルインストール

すべてのプロジェクトで使用するために archlint をグローバルにインストールする場合：

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

グローバルインストール後は、`archlint` を直接実行できます：

```bash
archlint scan
```

### ローカルインストール

あるいは、プロジェクトのデヴ依存関係（dev dependency）としてインストールすることもできます：

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

### ソースからインストール (Rust)

ネイティブバイナリを直接使用したい場合は、Cargo 経由でインストールできます：

```bash
cargo install archlint
```

## ESLint プラグイン

IDE でリアルタイムのアーキテクチャ・フィードバックを得るには、ESLint プラグインをインストールします：

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

設定の詳細については、[ESLint 統合](/ja/integrations/eslint)のセクションを参照してください。

## MCP サーバー

Claude や Cursor などの AI コーディングアシスタントを使用している場合は、当社の MCP サーバーをインストールできます：

```bash
npx @archlinter/mcp-server
```

詳細については、[MCP サーバー](/ja/integrations/mcp-server)のセクションを参照してください。

## GitHub アクション

プルリクエストでのアーキテクチャの退行を防ぐには、公式の GitHub アクションを使用してください：

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

詳細については、[GitHub アクション](/ja/integrations/github-actions)のセクションを参照してください。
