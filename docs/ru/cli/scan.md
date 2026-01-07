# archlint scan

Команда `scan` выполняет полный архитектурный анализ вашего проекта.

## Использование

```bash
archlint scan [path] [options]
```

## Опции

| Опция                       | По умолчанию | Описание                                                   |
| --------------------------- | ------------ | ---------------------------------------------------------- |
| `--format <format>`         | `table`      | Формат вывода: `table`, `json`, `markdown`                 |
| `--report <file>`           | `stdout`     | Сохранить отчет в файл                                     |
| `--min-severity <sev>`      | `low`        | Фильтр по критичности: `low`, `medium`, `high`, `critical` |
| `--detectors <ids>`         | `all`        | Список детекторов через запятую для запуска                |
| `--exclude-detectors <ids>` | `none`       | Детекторы, которые следует пропустить                      |
| `--no-cache`                | `false`      | Отключить кэширование анализа                              |

## Примеры

### Scan с отчетом в формате Markdown

```bash
archlint scan --format markdown --report report.md
```

### Запуск только детектора циклов

```bash
archlint scan --detectors cycles,circular_type_deps
```

### Только высокая критичность

```bash
archlint scan --min-severity high
```
