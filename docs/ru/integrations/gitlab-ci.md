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

## Лучшие практики

1. **Используйте `diff`**: Всегда сравнивайте с целевой веткой, чтобы сфокусироваться на новых проблемах.
2. **Падайте раньше**: Используйте `--fail-on`, чтобы гарантировать, что регрессии не попадут в основную ветку.
3. **Изучайте объяснения**: Вывод `--explain` помогает разработчикам понять, как исправить регрессии, не выходя из MR.
