---
title: GitLab CI
description: "Imponha regras arquiteturais em seus Merge Requests do GitLab usando o archlint no seu pipeline de CI/CD."
---

# GitLab CI

Imponha regras arquiteturais em seus Merge Requests do GitLab.

## Exemplo de `.gitlab-ci.yml`

```yaml
architecture_check:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli diff $CI_MERGE_REQUEST_TARGET_BRANCH_NAME --fail-on medium --explain
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

## Melhores Práticas

1. **Use `diff`**: Sempre compare com o branch de destino para focar em novos problemas.
2. **Falhe cedo**: Use `--fail-on` para garantir que nenhuma regressão chegue ao branch principal.
3. **Revise as Explicações**: A saída do `--explain` ajuda os desenvolvedores a entender como corrigir as regressões sem sair do MR.
