# Servidor MCP

O archlint fornece um servidor MCP (Model Context Protocol), permitindo que assistentes de codificação por IA como Claude ou Cursor entendam e melhorem sua arquitetura.

## Por que usar o Servidor MCP?

- **Refatoração com IA**: Seu assistente de IA pode ver os "cheiros" (smells) arquiteturais e sugerir mudanças específicas no código para corrigi-los.
- **Conhecimento Contextual**: O assistente pode perguntar "Por que este é um God Module?" e obter uma resposta detalhada baseada na análise real.
- **Correções Automatizadas**: Peça ao assistente para "Corrigir todas as dependências circulares nesta pasta" e ele poderá usar a análise do archlint para realizar a refatoração.

## Instalação

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

## Adicionar Rapidamente ao Cursor

Se você usa o [Cursor](https://cursor.com), pode adicionar o servidor MCP com um único clique:

<a href="cursor://anysphere.cursor-deeplink/mcp/install?name=archlint&config=eyJjb21tYW5kIjoibnB4IiwiYXJncyI6WyIteSIsIkBhcmNobGludGVyL21jcC1zZXJ2ZXIiXX0=" class="add-to-cursor-btn">
  <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <path d="M11.503.131 1.891 5.678a.84.84 0 0 0-.42.726v11.188c0 .3.162.575.42.724l9.609 5.55a1 1 0 0 0 .998 0l9.61-5.55a.84.84 0 0 0 .42-.724V6.404a.84.84 0 0 0-.42-.726L12.497.131a1.01 1.01 0 0 0-.996 0M2.657 6.338h18.55c.263 0 .43.287.297.515L12.23 22.918c-.062.107-.229.064-.229-.06V12.335a.59.59 0 0 0-.295-.51l-9.11-5.257c-.109-.063-.064-.23.061-.23" fill="currentColor"/>
  </svg>
  Adicionar ao Cursor
</a>

## Configuração Manual (Cursor/Claude Desktop)

Adicione o seguinte às suas configurações de MCP:

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

## Ferramentas Disponíveis

O servidor MCP expõe várias ferramentas para a IA:

- `archlint_scan`: Realiza uma varredura completa e retorna uma lista de smells.
- `archlint_explain`: Explica um smell específico e fornece conselhos de refatoração.
- `archlint_stats`: Fornece métricas arquiteturais de alto nível para o projeto.
