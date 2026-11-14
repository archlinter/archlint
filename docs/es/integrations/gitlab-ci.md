---
title: GitLab CI
description: 'Aplica reglas arquitectónicas en los Merge Requests de GitLab y publica los hallazgos de archlint como un informe de tests JUnit.'
---

# GitLab CI

Aplica reglas arquitectónicas en tus Merge Requests de GitLab.

## Ejemplo de `.gitlab-ci.yml`

```yaml
architecture_check:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli diff $CI_MERGE_REQUEST_TARGET_BRANCH_NAME --fail-on medium --explain
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

## Informes de tests JUnit

`archlint scan --format junit` imprime el XML en stdout. Solo escribe un archivo si se pasa `--report`. `archlint scan --format junit --report archlint-junit.xml` escribe el archivo que GitLab recoge con [`artifacts:reports:junit`](https://docs.gitlab.com/ci/testing/unit_test_reports/). GitLab muestra los resultados en la pestaña **Tests** del pipeline y en el resumen de tests del merge request.

Cada detector es un `<testsuite>`. Cada smell es un `<testcase>` fallido:

| Campo                            | Valor                                                |
| -------------------------------- | ---------------------------------------------------- |
| Nombre de la suite / `classname` | Id del detector, por ejemplo `dead_symbols`          |
| `name`                           | Explicación, más el archivo y la línea principales   |
| `file` y `line`                  | Ubicación principal, relativa a la raíz del proyecto |
| `<failure type>`                 | Severidad: `low`, `medium`, `high` o `critical`      |
| Cuerpo del fallo                 | Motivo, riesgos, recomendaciones y ubicaciones       |

Un scan sin smells emite un único caso de prueba que pasa, llamado `no architectural smells`.

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

`when: always` sube el informe cuando el job falla. `paths` deja el XML descargable entre los artefactos del job. Añade el mismo bloque `rules` que el job de diff si el informe solo debe ejecutarse en merge requests.

### Estado del job

El archivo JUnit registra cada smell como un test fallido. El estado del job sigue el código de salida de `scan`:

| Hallazgos                                      | Código de salida |
| ---------------------------------------------- | ---------------- |
| Cualquier smell `critical`                     | `2`              |
| Cualquier smell `high`, y ninguno `critical`   | `1`              |
| Solo smells `low` o `medium`, o un scan limpio | `0`              |

Los smells low y medium aparecen como tests fallidos y el job sigue en verde. Pasa `--min-severity high` cuando el informe solo debe listar hallazgos que también hacen fallar el job:

```bash
npx @archlinter/cli scan --format junit --min-severity high --report archlint-junit.xml
```

Usa `diff --fail-on` cuando el pipeline deba bloquear regresiones nuevas en lugar del inventario completo.

## Mejores Prácticas

1. **Usa `diff`**: Compara siempre contra la rama de destino para centrarte en los nuevos problemas.
2. **Falla pronto**: Usa `--fail-on` para asegurar que ninguna regresión llegue a la rama principal.
3. **Revisa las explicaciones**: La salida de `--explain` ayuda a los desarrolladores a entender cómo solucionar las regresiones sin salir del MR.
4. **Publica un informe JUnit**: Recoge `--format junit` con `artifacts:reports:junit` para que los hallazgos aparezcan en la pestaña Tests del pipeline y en el merge request.
