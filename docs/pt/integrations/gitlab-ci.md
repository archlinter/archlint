---
title: GitLab CI
description: 'Imponha regras arquiteturais nos Merge Requests do GitLab e publique os achados do archlint como um relatório de testes JUnit.'
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

## Relatórios de testes JUnit

`archlint scan --format junit` imprime o XML no stdout. Ele grava um arquivo apenas quando se passa `--report`. `archlint scan --format junit --report archlint-junit.xml` grava o arquivo que o GitLab coleta com [`artifacts:reports:junit`](https://docs.gitlab.com/ci/testing/unit_test_reports/). O GitLab mostra os resultados na aba **Tests** do pipeline e no resumo de testes do merge request.

Cada detector vira um `<testsuite>`. Cada smell vira um `<testcase>` com falha:

| Campo                       | Valor                                             |
| --------------------------- | ------------------------------------------------- |
| Nome da suíte / `classname` | Id do detector, como `dead_symbols`               |
| `name`                      | Explicação, mais o arquivo e a linha principais   |
| `file` e `line`             | Localização principal, relativa à raiz do projeto |
| `<failure type>`            | Severidade: `low`, `medium`, `high` ou `critical` |
| Corpo da falha              | Motivo, riscos, recomendações e localizações      |

Um scan sem smells emite um único caso de teste que passa, chamado `no architectural smells`.

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

`when: always` envia o relatório quando o job falha. `paths` deixa o XML disponível para download nos artefatos do job. Adicione o mesmo bloco `rules` do job de diff quando o relatório só deve rodar em merge requests.

### Status do job

O arquivo JUnit registra cada smell como um teste com falha. O status do job segue o código de saída do `scan`:

| Achados                                           | Código de saída |
| ------------------------------------------------- | --------------- |
| Qualquer smell `critical`                         | `2`             |
| Qualquer smell `high`, e nenhum `critical`        | `1`             |
| Apenas smells `low` ou `medium`, ou um scan limpo | `0`             |

Smells low e medium aparecem como testes com falha, e o job continua verde. Passe `--min-severity high` quando o relatório deve listar só achados que também fazem o job falhar:

```bash
npx @archlinter/cli scan --format junit --min-severity high --report archlint-junit.xml
```

Use `diff --fail-on` quando o pipeline deve bloquear regressões novas em vez do inventário completo.

## Melhores Práticas

1. **Use `diff`**: Sempre compare com o branch de destino para focar em novos problemas.
2. **Falhe cedo**: Use `--fail-on` para garantir que nenhuma regressão chegue ao branch principal.
3. **Revise as Explicações**: A saída do `--explain` ajuda os desenvolvedores a entender como corrigir as regressões sem sair do MR.
4. **Publique um relatório JUnit**: Colete `--format junit` com `artifacts:reports:junit` para que os achados apareçam na aba Tests do pipeline e no merge request.
