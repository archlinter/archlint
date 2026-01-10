# Пресеты фреймворков

archlint использует пресеты на базе YAML, чтобы понимать специфичные для фреймворков паттерны и уменьшать количество ложных срабатываний.

## Как это работает

archlint автоматически определяет фреймворки, анализируя зависимости в `package.json` и конфигурационные файлы. Вы также можете явно указать пресеты в вашем `.archlint.yaml`:

```yaml
extends:
  - nestjs
  - ./my-company-preset.yaml
```

## Встроенные пресеты

- **nestjs**: Для приложений NestJS.
- **nextjs**: Для проектов Next.js.
- **react**: Для библиотек и приложений React.
- **oclif**: Для CLI инструментов, созданных с помощью oclif.

## Кастомные пресеты

Файл пресета — это стандартный файл конфигурации archlint с дополнительной секцией `detect` для автоопределения.

### Структура

```yaml
name: my-framework
version: 1

# Правила для автоопределения
detect:
  packages:
    any_of: ['my-core-pkg']
  files:
    any_of: ['my-framework.config.js']

# Глобальные правила
rules:
  layer_violation: high
  dead_symbols:
    ignore_methods: ['onInit', 'onDestroy']
  vendor_coupling:
    ignore_packages: ['my-framework/*']

# Переопределения по путям
overrides:
  - files: ['**/*.controller.ts']
    rules:
      lcom: off

# Паттерны для анализа мертвого кода
entry_points:
  - '**/*.controller.ts'
```

### Загрузка кастомных пресетов

Вы можете загружать пресеты из локальных файлов или по URL:

```yaml
extends:
  - ./presets/shared.yaml
  - https://raw.githubusercontent.com/org/archlint-presets/main/standard.yaml
```

## Логика объединения

Пресеты объединяются в том порядке, в котором они указаны. Приоритет следующий:

1. Конфигурация пользователя в `.archlint.yaml` (самый высокий)
2. Пресеты из списка `extends`
3. Автоматически определенные пресеты
4. Настройки archlint по умолчанию (самый низкий)

Для списков (таких, как `entry_points` или `ignore_packages` внутри правил) archlint выполняет объединение (union) всех значений. Правила и переопределения объединяются рекурсивно.
