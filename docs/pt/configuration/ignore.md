# Padrões de Ignore

O archlint oferece várias maneiras de excluir arquivos ou diretórios da análise.

## Ignore Global

A seção `ignore` no `archlint.yaml` especifica arquivos que devem ser completamente pulados por todos os detectores.

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## Suporte ao .gitignore

Por padrão, o archlint respeita automaticamente o seu arquivo `.gitignore`. Você não precisa duplicar esses padrões no seu `archlint.yaml`.

## Ignore Específico por Detector

Alguns detectores têm seus próprios `exclude_patterns` dentro da seção `thresholds`. Isso é útil se você deseja que um arquivo seja analisado pela maioria dos detectores, mas pulado por um específico (por exemplo, excluindo arquivos de teste da detecção de ciclos).

```yaml
thresholds:
  cycles:
    exclude_patterns:
      - '**/*.test.ts'
      - '**/*.spec.ts'
```

## Ignores Inline

(Em breve) Estamos trabalhando para oferecer suporte a comentários inline como `// archlint-disable` para ignorar linhas ou arquivos específicos diretamente no código-fonte.
