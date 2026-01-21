---
title: Установка
description: "Установите archlint как инструмент CLI через npx или npm, или используйте его как плагин ESLint для обратной связи в реальном времени в вашем редакторе."
---

# Установка

archlint можно использовать как инструмент командной строки (CLI) или как плагин ESLint.

## Инструмент CLI (рекомендуется)

Самый простой способ использовать archlint — через `npx`. Это гарантирует, что вы всегда используете последнюю версию, не добавляя её в свой `package.json`.

```bash
npx @archlinter/cli scan
```

### Глобальная установка

Если вы хотите установить archlint глобально для использования во всех проектах:

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

После глобальной установки вы можете запускать `archlint` напрямую:

```bash
archlint scan
```

### Локальная установка

В качестве альтернативы вы можете установить его как зависимость для разработки в вашем проекте:

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

### Из исходного кода (Rust)

Если вы предпочитаете использовать нативный бинарный файл напрямую, вы можете установить его через Cargo:

```bash
cargo install archlint
```

## Плагин ESLint

Чтобы получать архитектурную обратную связь в режиме реального времени в вашей IDE, установите плагин ESLint:

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

Подробности конфигурации см. в разделе [Интеграция с ESLint](/ru/integrations/eslint).

## MCP сервер

Если вы используете AI-помощников, таких как Claude или Cursor, вы можете установить наш MCP-сервер:

```bash
npx @archlinter/mcp-server
```

Дополнительную информацию см. в разделе [MCP сервер](/ru/integrations/mcp-server).

## GitHub Action

Чтобы предотвратить архитектурные регрессии в ваших Pull Requests, используйте наш официальный GitHub Action:

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

Дополнительную информацию см. в разделе [GitHub Actions](/ru/integrations/github-actions).
