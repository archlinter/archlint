# Слои

Конфигурация слоев позволяет определить архитектурные уровни вашего проекта и обеспечить соблюдение правил зависимостей между ними.

## Определение слоев

Настройка слоев производится внутри правила `layer_violation`. Каждое определение слоя состоит из:

- `name`: Уникальное имя слоя.
- `path` (или `paths`): Glob-паттерн, идентифицирующий файлы в этом слое.
- `allowed_imports` (или `can_import`): Список имен слоев, которые этот слой может импортировать.

## Пример: Чистая архитектура (Clean Architecture)

```yaml
rules:
  layer_violation:
    severity: high
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: [] # Слой Domain не должен ни от чего зависеть

      - name: application
        path: '**/application/**'
        allowed_imports:
          - domain

      - name: infrastructure
        path: '**/infrastructure/**'
        allowed_imports:
          - domain
          - application

      - name: presentation
        path: '**/presentation/**'
        allowed_imports:
          - domain
          - application
```

## Как это работает

Когда включен детектор `layer_violation`:

1. Он сопоставляет каждый файл в вашем проекте с определенным слоем на основе паттерна `path`.
2. Если файл попадает под несколько паттернов, выбирается самый специфичный (наиболее длинный паттерн).
3. Инструмент проверяет каждый импорт. Если файл в слое `A` импортирует файл в слое `B`, но `B` не указан в списке `allowed_imports` слоя `A`, сообщается о нарушении.
