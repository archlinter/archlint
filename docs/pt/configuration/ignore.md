# Ignorando Arquivos

O archlint fornece várias maneiras de excluir arquivos ou diretórios da análise.

## Ignorar Global

A seção `ignore` na raiz do `.archlint.yaml` especifica arquivos que devem ser completamente ignorados por todos os detectores.

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## Suporte ao .gitignore

Por padrão, o archlint respeita automaticamente seu arquivo `.gitignore`. Você não precisa duplicar esses padrões no `.archlint.yaml`. Se desejar desativar esse comportamento, defina `git: { enabled: false }`.

## Ignorar por Regra

Você pode excluir arquivos de um detector específico usando o campo `exclude` dentro da seção `rules`. Isso é útil se você deseja que um arquivo seja analisado pela maioria dos detectores, mas ignorado por um detector específico.

```yaml
rules:
  cycles:
    exclude:
      - '**/generated/**'
      - '**/*.entity.ts'
```

## Substituições de Caminho (Overrides)

Para lógica mais complexa (por exemplo, alterar configurações ou desativar várias regras para um diretório específico), use a seção `overrides`:

```yaml
overrides:
  - files: ['**/tests/**', '**/mocks/**']
    rules:
      complexity: off
      god_module: off
      large_file: medium
```

## Ignorar Inline

(Em Desenvolvimento) Estamos trabalhando no suporte a comentários como `// archlint-disable` para ignorar linhas ou arquivos específicos diretamente no código.
