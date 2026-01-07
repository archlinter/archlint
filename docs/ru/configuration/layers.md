# Слои

Конфигурация `layers` позволяет определить архитектурные слои вашего проекта и обеспечить соблюдение правил зависимостей между ними.

## Определение слоев

Каждое определение слоя состоит из:

- `name`: Уникальный идентификатор слоя.
- `paths`: Массив glob-паттернов, идентифицирующих файлы в этом слое.
- `can_import`: Массив имен слоев, от которых этот слой может зависеть.

## Пример: Clean Architecture

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Слой Domain должен быть независимым

  - name: application
    paths: ['**/application/**', '**/use-cases/**']
    can_import:
      - domain

  - name: infrastructure
    paths: ['**/infrastructure/**', '**/adapters/**']
    can_import:
      - domain
      - application

  - name: presentation
    paths: ['**/controllers/**', '**/api/**', '**/ui/**']
    can_import:
      - domain
      - application
```

## Как это работает

Когда включен детектор `layer_violation`:

1. Он назначает каждый файл в вашем проекте определенному слою на основе паттернов `paths`.
2. Он проверяет каждый импорт в этих файлах.
3. Если файл в слое `A` импортирует файл в слое `B`, но `B` не указан в списке `can_import` слоя `A`, сообщается о нарушении.
