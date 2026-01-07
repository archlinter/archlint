# Servidor MCP

archlint proporciona un servidor MCP (Model Context Protocol), lo que permite que los asistentes de codificación por IA como Claude o Cursor comprendan y mejoren tu arquitectura.

## ¿Por qué usar el Servidor MCP?

- **Refactorización Impulsada por IA**: Tu asistente de IA puede ver los smells arquitectónicos y sugerir cambios de código específicos para solucionarlos.
- **Conocimiento Contextual**: El asistente puede preguntar "¿Por qué es esto un God Module?" y obtener una respuesta detallada basada en el análisis real.
- **Correcciones Automatizadas**: Pide al asistente que "Corrija todas las dependencias circulares en esta carpeta" y podrá usar el análisis de archlint para realizar la refactorización.

## Instalación

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

## Adición Rápida a Cursor

Si usas [Cursor](https://cursor.com), puedes añadir el servidor MCP con un solo clic:

<a href="cursor://anysphere.cursor-deeplink/mcp/install?name=archlint&config=eyJjb21tYW5kIjoibnB4IiwiYXJncyI6WyIteSIsIkBhcmNobGludGVyL21jcC1zZXJ2ZXIiXX0=" class="add-to-cursor-btn">
  <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <path d="M11.503.131 1.891 5.678a.84.84 0 0 0-.42.726v11.188c0 .3.162.575.42.724l9.609 5.55a1 1 0 0 0 .998 0l9.61-5.55a.84.84 0 0 0 .42-.724V6.404a.84.84 0 0 0-.42-.726L12.497.131a1.01 1.01 0 0 0-.996 0M2.657 6.338h18.55c.263 0 .43.287.297.515L12.23 22.918c-.062.107-.229.064-.229-.06V12.335a.59.59 0 0 0-.295-.51l-9.11-5.257c-.109-.063-.064-.23.061-.23" fill="currentColor"/>
  </svg>
  Add to Cursor
</a>

## Configuración Manual (Cursor/Claude Desktop)

Añade lo siguiente a tu configuración de MCP:

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

## Herramientas Disponibles

El servidor MCP expone varias herramientas a la IA:

- `archlint_scan`: Realiza un escaneo completo y devuelve una lista de smells.
- `archlint_explain`: Explica un smell específico y proporciona consejos de refactorización.
- `archlint_stats`: Proporciona métricas arquitectónicas de alto nivel para el proyecto.
