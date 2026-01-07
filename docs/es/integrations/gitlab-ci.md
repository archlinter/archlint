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

## Mejores Prácticas

1. **Usa `diff`**: Compara siempre contra la rama de destino para centrarte en los nuevos problemas.
2. **Falla pronto**: Usa `--fail-on` para asegurar que ninguna regresión llegue a la rama principal.
3. **Revisa las explicaciones**: La salida de `--explain` ayuda a los desarrolladores a entender cómo solucionar las regresiones sin salir del MR.
