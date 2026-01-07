---
title: Конфигурация
description: Узнайте, как настроить archlint с помощью archlint.yaml, определить архитектурные слои и установить пользовательские пороги для детекторов.
---

# Конфигурация

archlint можно настроить с помощью файла `archlint.yaml` в корне вашего проекта. Если конфигурационный файл не найден, инструмент использует разумные значения по умолчанию для всех детекторов.

## Структура конфигурационного файла

```yaml
# Файлы для игнорирования
ignore:
  - '**/dist/**'

# Алиасы путей (например, из tsconfig.json)
aliases:
  '@/*': 'src/*'

# Точки входа для анализа мертвого кода
entry_points:
  - 'src/index.ts'

# Пользовательские пороги для детекторов
thresholds:
  cycles:
    exclude_patterns: []
  god_module:
    fan_in: 15
    fan_out: 15

# Архитектурные слои
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: []

# Пресеты фреймворков
frameworks:
  - nestjs

# Переопределение критичности
severity:
  cycles: critical
```

## Конфигурация через CLI

Вы также можете указать путь к конфигурационному файлу через CLI:

```bash
archlint scan --config custom-config.yaml
```
