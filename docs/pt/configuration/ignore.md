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
      cyclomatic_complexity: off
      god_module: off
      large_file: medium
```

## Ignorar Inline

Você pode ignorar problemas arquiteturais específicos diretamente no seu código-fonte usando comentários especiais. Isso é útil para suprimir avisos em casos excepcionais.

### Uso:

Tanto a sintaxe de comentário de linha única (`// archlint-...`) quanto a de comentário de bloco (`/* archlint-... */`) são suportadas para todos os padrões.

1. **Todo o arquivo**: Adicione `// archlint-disable` no início do arquivo.
2. **Linha atual**: Adicione `// archlint-disable-line` no final da linha ou na linha acima.
3. **Próxima linha**: Use `// archlint-disable-next-line` antes da linha problemática.
4. **Blocos**: Use `// archlint-disable` e `// archlint-enable` para envolver uma seção de código.

### Exemplos:

```typescript
// archlint-disable * - Todo o arquivo usa padrões legados
// Ignorar todas as regras para todo o arquivo

// prettier-ignore
// archlint-disable-next-line long-params - Esta função legada requer muitos parâmetros
function processTransaction(id: string, amount: number, currency: string, date: Date, recipient: string, note: string) {
  // O detector de parâmetros longos será ignorado apenas para esta linha
}

import { internal } from './private'; // archlint-disable-line layer_violation - Exclusão temporária para migração

/* archlint-disable cyclomatic_complexity, cognitive_complexity */
function legacyCode() {
  // Este bloco será ignorado para ambos os tipos de complexidade
}
/* archlint-enable cyclomatic_complexity, cognitive_complexity */
```

Você pode especificar várias regras separadas por vírgulas ou usar `*` para ignorar todas as regras.
