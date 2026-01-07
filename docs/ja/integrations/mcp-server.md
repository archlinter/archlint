# MCPサーバー

archlintはMCP（Model Context Protocol）サーバーを提供しており、ClaudeやCursorなどのAIコーディングアシスタントがプロジェクトのアーキテクチャを理解し、改善できるようにします。

## なぜMCPサーバーを使用するのですか？

- **AI駆動のリファクタリング**: AIアシスタントがアーキテクチャ上の不吉なにおい（smells）を認識し、それらを修正するための具体的なコード変更を提案できます。
- **文脈に応じた知識**: アシスタントは「なぜこれがGodモジュールなのですか？」と質問し、実際の分析に基づいた詳細な回答を得ることができます。
- **自動化された修正**: アシスタントに「このフォルダー内のすべての循環依存関係を修正してください」と依頼すると、archlintの分析を使用してリファクタリングを実行できます。

## インストール

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

## Cursorへのクイック追加

[Cursor](https://cursor.com) を使用している場合は、ワンクリックでMCPサーバーを追加できます：

<a href="cursor://anysphere.cursor-deeplink/mcp/install?name=archlint&config=eyJjb21tYW5kIjoibnB4IiwiYXJncyI6WyIteSIsIkBhcmNobGludGVyL21jcC1zZXJ2ZXIiXX0=" class="add-to-cursor-btn">
  <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <path d="M11.503.131 1.891 5.678a.84.84 0 0 0-.42.726v11.188c0 .3.162.575.42.724l9.609 5.55a1 1 0 0 0 .998 0l9.61-5.55a.84.84 0 0 0 .42-.724V6.404a.84.84 0 0 0-.42-.726L12.497.131a1.01 1.01 0 0 0-.996 0M2.657 6.338h18.55c.263 0 .43.287.297.515L12.23 22.918c-.062.107-.229.064-.229-.06V12.335a.59.59 0 0 0-.295-.51l-9.11-5.257c-.109-.063-.064-.23.061-.23" fill="currentColor"/>
  </svg>
  Add to Cursor
</a>

## 手動設定 (Cursor/Claude Desktop)

MCP設定に以下を追加してください：

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

## 利用可能なツール

MCPサーバーは、AIに対して以下のツールを公開しています：

- `archlint_scan`: フルスキャンを実行し、不吉なにおいのリストを返します。
- `archlint_explain`: 特定の不吉なにおいについて説明し、リファクタリングのアドバイスを提供します。
- `archlint_stats`: プロジェクトのハイレベルなアーキテクチャメトリクスを提供します。
