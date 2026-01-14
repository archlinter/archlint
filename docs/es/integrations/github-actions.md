# GitHub Actions

Integra archlint en tu flujo de trabajo de GitHub para prevenir regresiones arquitectónicas en cada Pull Request con comentarios y anotaciones detalladas.

## La Acción de archlint (Action)

La forma más sencilla de usar archlint en GitHub es a través de nuestra [GitHub Action](https://github.com/marketplace/actions/archlint) oficial.

### Características

- **Comentarios en PR**: Publica automáticamente un informe detallado en tu PR.
- **Anotaciones en línea**: Muestra regresiones arquitectónicas directamente en las líneas de código que las causaron.
- **Resumen Automático**: Añade un informe resumen a la página de ejecución del job.

### Ejemplo de Flujo de Trabajo (Workflow)

Crea `.github/workflows/architecture.yml`:

<div v-pre>

```yaml
name: Architecture

on: [pull_request]

jobs:
  archlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write # Requerido para comentarios en PR
      security-events: write # Requerido para cargar SARIF
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Importante para el análisis de git diff analysis

      - name: archlint
        uses: archlinter/action@v1
        with:
          baseline: origin/${{ github.base_ref }}
          fail-on: medium
          github-token: ${{ github.token }}
```

</div>

## Entradas (Inputs)

<div v-pre>

| Entrada             | Descripción                                                        | Por defecto           |
| ------------------- | ------------------------------------------------------------------ | --------------------- |
| `baseline`          | Referencia de Git o archivo snapshot para comparar                 | `origin/main`         |
| `fail-on`           | Severidad mínima para fallar (`low`, `medium`, `high`, `critical`) | `medium`              |
| `comment`           | Publica un comentario en el PR con el informe de arquitectura      | `true`                |
| `annotations`       | Muestra anotaciones en línea para los smells arquitectónicos       | `true`                |
| `working-directory` | Directorio a analizar                                              | `.`                   |
| `github-token`      | Token de GitHub para publicar comentarios                          | `${{ github.token }}` |

</div>

## Uso Manual de la CLI

Si prefieres ejecutar la CLI manualmente, puedes usar `npx @archlinter/cli`:

<div v-pre>

```yaml
- name: Check for architectural regressions
  run: npx @archlinter/cli diff origin/${{ github.base_ref }} --fail-on medium --explain
```

</div>

### Flags de la CLI para CI

- `--fail-on <severity>`: Sale con código 1 si se encuentran regresiones de este nivel o superior.
- `--explain`: Consejos detallados sobre por qué un smell es malo y cómo solucionarlo.
- `--json`: Salida del resultado en formato JSON para procesamiento personalizado.
- `--format sarif`: Salida en formato SARIF para integración con GitHub Code Scanning.

## Integración con GitHub Code Scanning

Puedes cargar los resultados de archlint en GitHub Code Scanning para ver los problemas arquitectónicos en la pestaña "Security" (Seguridad) y como anotaciones de PR.

```yaml
- name: Scan architecture
  run: npx @archlinter/cli scan --format sarif --report archlint.sarif

- name: Upload SARIF file
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: archlint.sarif
```
