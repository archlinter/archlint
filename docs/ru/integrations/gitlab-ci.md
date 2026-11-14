---
title: GitLab CI
description: 'Соблюдайте архитектурные правила в GitLab Merge Requests и публикуйте находки archlint как отчёт о тестах JUnit.'
---

# GitLab CI

Обеспечьте соблюдение архитектурных правил в ваших GitLab Merge Requests.

## Пример `.gitlab-ci.yml`

```yaml
architecture_check:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli diff $CI_MERGE_REQUEST_TARGET_BRANCH_NAME --fail-on medium --explain
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

## Отчёты о тестах JUnit

`archlint scan --format junit` печатает XML в stdout. Файл записывается только с `--report`. `archlint scan --format junit --report archlint-junit.xml` записывает файл, который GitLab собирает через [`artifacts:reports:junit`](https://docs.gitlab.com/ci/testing/unit_test_reports/). GitLab показывает результаты на вкладке **Tests** пайплайна и в сводке тестов merge request.

Каждый детектор становится `<testsuite>`. Каждый smell становится упавшим `<testcase>`:

| Поле                     | Значение                                            |
| ------------------------ | --------------------------------------------------- |
| Имя набора / `classname` | Id детектора, например `dead_symbols`               |
| `name`                   | Объяснение плюс основной файл и строка              |
| `file` и `line`          | Основное место, относительно корня проекта          |
| `<failure type>`         | Серьёзность: `low`, `medium`, `high` или `critical` |
| Тело ошибки              | Причина, риски, рекомендации и места                |

Scan без smell выдаёт один прошедший тест с именем `no architectural smells`.

```yaml
architecture_report:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli scan --format junit --report archlint-junit.xml
  artifacts:
    when: always
    paths:
      - archlint-junit.xml
    reports:
      junit: archlint-junit.xml
```

`when: always` загружает отчёт, даже если job упал. `paths` оставляет XML доступным для скачивания среди артефактов job. Добавьте тот же блок `rules`, что и у job с diff, если отчёт нужен только в merge request.

### Статус job

Файл JUnit записывает каждый smell как упавший тест. Статус job следует коду выхода `scan`:

| Находки                                           | Код выхода |
| ------------------------------------------------- | ---------- |
| Любой smell `critical`                            | `2`        |
| Любой smell `high` и ни одного `critical`         | `1`        |
| Только smell `low` или `medium`, либо чистый scan | `0`        |

Smell уровня low и medium видны как упавшие тесты, а job остаётся зелёным. Передайте `--min-severity high`, если отчёт должен содержать только находки, которые также роняют job:

```bash
npx @archlinter/cli scan --format junit --min-severity high --report archlint-junit.xml
```

Используйте `diff --fail-on`, когда пайплайн должен блокировать новые регрессии, а не весь инвентарь.

## Лучшие практики

1. **Используйте `diff`**: Всегда сравнивайте с целевой веткой, чтобы сфокусироваться на новых проблемах.
2. **Падайте раньше**: Используйте `--fail-on`, чтобы гарантировать, что регрессии не попадут в основную ветку.
3. **Изучайте объяснения**: Вывод `--explain` помогает разработчикам понять, как исправить регрессии, не выходя из MR.
4. **Публикуйте отчёт JUnit**: Собирайте `--format junit` через `artifacts:reports:junit`, чтобы находки появлялись на вкладке Tests пайплайна и в merge request.
