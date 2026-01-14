# GitHub Actions

Интегрируйте archlint в ваш рабочий процесс GitHub, чтобы предотвращать архитектурные регрессии в каждом Pull Request с помощью красивых комментариев и аннотаций.

## Экшен archlint

Самый простой способ использовать archlint на GitHub — через наш официальный [GitHub Action](https://github.com/marketplace/actions/archlint).

### Возможности

- **Комментарии к PR**: Автоматически публикует подробный отчет в вашем PR.
- **Инлайновые аннотации**: Показывает архитектурную регрессию прямо на строках кода, которые их вызвали.
- **Автоматическое резюме**: Добавляет сводный отчет на страницу выполнения задания.

### Пример Workflow

Создайте `.github/workflows/architecture.yml`:

<div v-pre>

```yaml
name: Architecture

on: [pull_request]

jobs:
  archlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write # Требуется для комментариев к PR
      security-events: write # Требуется для загрузки SARIF
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Важно для анализа git diff

      - name: archlint
        uses: archlinter/action@v1
        with:
          baseline: origin/${{ github.base_ref }}
          fail-on: medium
          github-token: ${{ github.token }}
```

</div>

## Входные параметры

<div v-pre>

| Параметр            | Описание                                                                  | По умолчанию          |
| ------------------- | ------------------------------------------------------------------------- | --------------------- |
| `baseline`          | Git ref или файл snapshot для сравнения                                   | `origin/main`         |
| `fail-on`           | Минимальная критичность для провала (`low`, `medium`, `high`, `critical`) | `medium`              |
| `comment`           | Публиковать комментарий в PR с отчетом                                    | `true`                |
| `annotations`       | Показывать инлайновые аннотации для архитектурных запахов                 | `true`                |
| `working-directory` | Директория для анализа                                                    | `.`                   |
| `github-token`      | Токен GitHub для публикации комментариев                                  | `${{ github.token }}` |

</div>

## Ручное использование CLI

Если вы предпочитаете запускать CLI вручную, вы можете использовать `npx @archlinter/cli`:

<div v-pre>

```yaml
- name: Check for architectural regressions
  run: npx @archlinter/cli diff origin/${{ github.base_ref }} --fail-on medium --explain
```

</div>

### Флаги CLI для CI

- `--fail-on <severity>`: Выйти с кодом 1, если найдены регрессии этого уровня или выше.
- `--explain`: Подробные советы о том, почему запах плох и как его исправить.
- `--json`: Вывод результата в формате JSON для кастомной обработки.
- `--format sarif`: Вывод в формате SARIF для интеграции с GitHub Code Scanning.

## Интеграция с GitHub Code Scanning

Вы можете загружать результаты archlint в GitHub Code Scanning, чтобы видеть архитектурные проблемы во вкладке "Security" и в виде аннотаций к PR.

```yaml
- name: Scan architecture
  run: npx @archlinter/cli scan --format sarif --report archlint.sarif

- name: Upload SARIF file
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: archlint.sarif
```
