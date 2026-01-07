# Código Morto (Dead Code)

**ID:** `dead_code` | **Gravidade:** Baixa (padrão)

Código morto refere-se a funções, classes ou variáveis exportadas que não são importadas ou usadas em nenhum outro lugar do projeto.

## Por que isso é um smell

- **Custo de Manutenção**: Desenvolvedores podem gastar tempo atualizando ou refatorando código que nem sequer é usado.
- **Tamanho do Bundle**: Aumenta o tamanho final da aplicação (embora muitos bundlers façam tree-shaking).
- **Confusão**: Faz com que a API de um módulo pareça maior e mais complexa do que realmente é.

## Exemplos

### Ruim

```typescript
// utils.ts
export const usedHelper = () => { ... };
export const unusedHelper = () => { ... }; // Relatado como código morto

// main.ts
import { usedHelper } from './utils';
```

## Como corrigir

1. **Delete-o**: Se estiver realmente sem uso, a melhor ação é a remoção.
2. **Marcar como Ponto de Entrada (Entry Point)**: Se for parte de uma API pública ou um import dinâmico, adicione-o a `entry_points` na sua configuração.

## Configuração

```yaml
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```
