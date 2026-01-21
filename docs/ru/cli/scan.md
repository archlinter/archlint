---
title: scan
description: "Выполните полный архитектурный анализ вашего проекта с помощью команды archlint scan, поддерживающей несколько форматов вывода, таких как JSON, Markdown и SARIF."
---

# archlint scan

Команда `scan` выполняет полный архитектурный анализ вашего проекта.

## Использование

```bash
archlint scan [path] [options]
```

## Опции

| Опция                           | По умолчанию | Описание                                                   |
| ------------------------------- | ------------ | ---------------------------------------------------------- |
| `-f, --format <format>`         | `table`      | Формат вывода: `table`, `json`, `markdown`, `sarif`        |
| `-j, --json`                    | `false`      | Сокращение для `--format json`                             |
| `-r, --report <file>`           | `stdout`     | Сохранить отчет в файл                                     |
| `-s, --min-severity <sev>`      | `low`        | Фильтр по критичности: `low`, `medium`, `high`, `critical` |
| `-S, --min-score <score>`       | `none`       | Фильтр по минимальному баллу здоровья (score)              |
| `-d, --detectors <ids>`         | `all`        | Список детекторов через запятую для запуска                |
| `-e, --exclude-detectors <ids>` | `none`       | Детекторы, которые следует пропустить                      |
| `-A, --all`                     | `false`      | Запустить все детекторы (включая выключенные по умолчанию) |
| `--no-cache`                    | `false`      | Отключить кэширование анализа                              |
| `--no-git`                      | `false`      | Отключить интеграцию с git (пропустить анализ churn)       |

## Примеры

### Scan с отчетом в формате Markdown

```bash
archlint scan --format markdown --report report.md
```

### Экспорт в SARIF (для GitHub Code Scanning)

```bash
archlint scan --format sarif --report results.sarif
```

### Запуск только детектора циклов

```bash
archlint scan --detectors cycles,circular_type_deps
```

### Только высокая критичность

```bash
archlint scan --min-severity high
```
