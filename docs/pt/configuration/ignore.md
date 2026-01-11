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

Você pode ignorar problemas arquiteturais específicos diretamente no seu código-fonte usando comentários especiais. Isso é útil para suprimir avisos em casos excepcionais.

### Uso:

1. **Todo o arquivo**: Adicione `// archlint-disable` no início do arquivo.
2. **Linha atual**: Adicione `// archlint-disable-line` no final da linha ou na linha acima.
3. **Próxima linha**: Use `// archlint-disable-next-line` antes da linha problemática.

### Exemplos:

```typescript
// archlint-disable-next-line complexity
function veryComplexFunction() {
  // O detector de complexidade será ignorado para esta função
}

import { internal } from './private'; // archlint-disable-line layer_violation

// archlint-disable cycles, god_module
// Ignorar regras específicas para todo o arquivo
```

Você pode especificar várias regras separadas por vírgulas ou usar `*` para ignorar todas as regras.
