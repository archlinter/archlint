---
title: Instalación
description: "Instala archlint como herramienta CLI vía npx o npm, o úsalo como plugin de ESLint para retroalimentación en tiempo real en tu editor."
---

# Instalación

archlint se puede utilizar como una herramienta de CLI o como un complemento de ESLint.

## Herramienta de CLI (Recomendado)

La forma más sencilla de utilizar archlint es a través de `npx`. Esto asegura que siempre estés utilizando la última versión sin tener que añadirla a tu `package.json`.

```bash
npx @archlinter/cli scan
```

### Instalación Global

Si deseas instalar archlint globalmente para usarlo en todos tus proyectos:

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

Tras la instalación global, puedes ejecutar `archlint` directamente:

```bash
archlint scan
```

### Instalación Local

Alternativamente, puedes instalarlo como una dependencia de desarrollo en tu proyecto:

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

### Desde el código fuente (Rust)

Si prefieres utilizar el binario nativo directamente, puedes instalarlo a través de Cargo:

```bash
cargo install archlint
```

## Plugin de ESLint

Para obtener comentarios arquitectónicos en tiempo real en tu IDE, instala el plugin de ESLint:

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

Consulta la sección de [Integración con ESLint](/es/integrations/eslint) para obtener detalles sobre la configuración.

## Servidor MCP

Si estás utilizando asistentes de codificación de IA como Claude o Cursor, puedes instalar nuestro servidor MCP:

```bash
npx @archlinter/mcp-server
```

Consulta la sección del [Servidor MCP](/es/integrations/mcp-server) para obtener más información.

## GitHub Action

Para evitar regresiones arquitectónicas en tus Pull Requests, utiliza nuestra GitHub Action oficial:

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

Consulta la sección de [GitHub Actions](/es/integrations/github-actions) para obtener más información.
