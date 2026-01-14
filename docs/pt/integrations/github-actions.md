# GitHub Actions

Integre o archlint ao seu workflow do GitHub para evitar regressões arquiteturais em cada Pull Request com comentários e anotações elegantes.

## A Action do archlint

A maneira mais fácil de usar o archlint no GitHub é através da nossa [GitHub Action](https://github.com/marketplace/actions/archlint) oficial.

### Recursos

- **Comentários em PR**: Posta automaticamente um relatório detalhado no seu PR.
- **Anotações Inline**: Mostra regressões arquiteturais diretamente nas linhas de código que as causaram.
- **Resumo Automático**: Adiciona um relatório de resumo à página de execução do job.

### Exemplo de Workflow

Crie `.github/workflows/architecture.yml`:

<div v-pre>

```yaml
name: Architecture

on: [pull_request]

jobs:
  archlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write # Necessário para comentários em PR
      security-events: write # Necessário para upload de SARIF
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Importante para análise de git diff

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

| Entrada             | Descrição                                                           | Padrão                |
| ------------------- | ------------------------------------------------------------------- | --------------------- |
| `baseline`          | Referência git ou arquivo de snapshot para comparar                 | `origin/main`         |
| `fail-on`           | Severidade mínima para falhar (`low`, `medium`, `high`, `critical`) | `medium`              |
| `comment`           | Postar comentário no PR com o relatório de arquitetura              | `true`                |
| `annotations`       | Mostrar anotações inline para "cheiros" (smells) arquiteturais      | `true`                |
| `working-directory` | Diretório para analisar                                             | `.`                   |
| `github-token`      | Token do GitHub para postar comentários                             | `${{ github.token }}` |

</div>

## Uso Manual via CLI

Se preferir executar a CLI manualmente, você pode usar `npx @archlinter/cli`:

<div v-pre>

```yaml
- name: Check for architectural regressions
  run: npx @archlinter/cli diff origin/${{ github.base_ref }} --fail-on medium --explain
```

</div>

### Flags da CLI para CI/CD

- `--fail-on <severity>`: Sai com 1 se regressões deste nível ou superior forem encontradas.
- `--explain`: Conselhos detalhados sobre por que um smell é ruim e como corrigi-lo.
- `--json`: Saída do resultado como JSON para processamento personalizado.
- `--format sarif`: Saída no formato SARIF para integração com o GitHub Code Scanning.

## Integração com GitHub Code Scanning

Você pode fazer upload dos resultados do archlint para o GitHub Code Scanning para ver os problemas arquiteturais na aba "Security" e como anotações em PRs.

```yaml
- name: Scan architecture
  run: npx @archlinter/cli scan --format sarif --report archlint.sarif

- name: Upload SARIF file
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: archlint.sarif
```
