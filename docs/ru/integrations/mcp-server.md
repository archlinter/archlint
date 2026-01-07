# MCP-сервер

archlint предоставляет сервер MCP (Model Context Protocol), позволяющий ИИ-помощникам, таким как Claude или Cursor, понимать и улучшать вашу архитектуру.

## Зачем использовать MCP-сервер?

- **Рефакторинг с помощью ИИ**: Ваш ИИ-помощник может видеть архитектурные запахи и предлагать конкретные изменения в коде для их исправления.
- **Контекстные знания**: Помощник может спросить "Почему это God Module?" и получить подробный ответ на основе реального анализа.
- **Автоматические исправления**: Попросите помощника "Исправить все циклические зависимости в этой папке", и он сможет использовать анализ archlint для выполнения рефакторинга.

## Установка

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

## Быстрое добавление в Cursor

Если вы используете [Cursor](https://cursor.com), вы можете добавить MCP-сервер одним кликом:

<a href="cursor://anysphere.cursor-deeplink/mcp/install?name=archlint&config=eyJjb21tYW5kIjoibnB4IiwiYXJncyI6WyIteSIsIkBhcmNobGludGVyL21jcC1zZXJ2ZXIiXX0=" class="add-to-cursor-btn">
  <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <path d="M11.503.131 1.891 5.678a.84.84 0 0 0-.42.726v11.188c0 .3.162.575.42.724l9.609 5.55a1 1 0 0 0 .998 0l9.61-5.55a.84.84 0 0 0 .42-.724V6.404a.84.84 0 0 0-.42-.726L12.497.131a1.01 1.01 0 0 0-.996 0M2.657 6.338h18.55c.263 0 .43.287.297.515L12.23 22.918c-.062.107-.229.064-.229-.06V12.335a.59.59 0 0 0-.295-.51l-9.11-5.257c-.109-.063-.064-.23.061-.23" fill="currentColor"/>
  </svg>
  Добавить в Cursor
</a>

## Ручная конфигурация (Cursor/Claude Desktop)

Добавьте следующее в настройки MCP:

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

## Доступные инструменты

MCP-сервер предоставляет ИИ несколько инструментов:

- `archlint_scan`: Выполняет полное сканирование и возвращает список запахов.
- `archlint_explain`: Объясняет конкретный запах и дает советы по рефакторингу.
- `archlint_stats`: Предоставляет высокоуровневые архитектурные метрики проекта.
