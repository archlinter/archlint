# Instalação

archlint pode ser usado como uma ferramenta CLI ou como um plugin ESLint.

## Ferramenta CLI (Recomendado)

A maneira mais fácil de usar o archlint é via `npx`. Isso garante que você esteja sempre usando a versão mais recente sem adicioná-la ao seu `package.json`.

```bash
npx @archlinter/cli scan
```

### Instalação Global

Se você deseja instalar o archlint globalmente para usar em todos os projetos:

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

Após a instalação global, você pode executar `archlint` diretamente:

```bash
archlint scan
```

### Instalação Local

Alternativamente, você pode instalá-lo como uma dependência de desenvolvimento em seu projeto:

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

### Da Fonte (Rust)

Se você preferir usar o binário nativo diretamente, pode instalá-lo via Cargo:

```bash
cargo install archlint
```

## Plugin ESLint

Para obter feedback arquitetural em tempo real no seu IDE, instale o plugin ESLint:

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

Consulte a seção [Integração ESLint](/pt/integrations/eslint) para detalhes de configuração.

## Servidor MCP

Se você estiver usando assistentes de codificação de IA como Claude ou Cursor, poderá instalar nosso servidor MCP:

```bash
npx @archlinter/mcp-server
```

Consulte a seção [Servidor MCP](/pt/integrations/mcp-server) para mais informações.

## GitHub Action

Para evitar regressões arquiteturais em seus Pull Requests, use nossa GitHub Action oficial:

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

Consulte a seção [GitHub Actions](/pt/integrations/github-actions) para mais informações.
