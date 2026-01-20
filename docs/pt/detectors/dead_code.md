# Código Morto (Dead Code)

**ID:** `dead_code` | **Gravidade:** Baixa (padrão)

Código morto é exatamente o que parece: funções, classes ou variáveis que estão "vivas" no seu código, mas que não fazem nada porque ninguém as usa.

## Por que isso é um smell

- **Desperdício de energia mental**: Os desenvolvedores não deveriam ter que refatorar ou tentar entender código que nem sequer está rodando.
- **Falsa complexidade**: Faz com que a API dos seus módulos pareça maior e mais assustadora do que realmente é.
- **Fantasmas no código**: Pode causar momentos de "eu pensei que já tínhamos removido isso" durante a depuração.

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
# Opções específicas da regra
rules:
  dead_code:
    exclude:
      - '**/tests/**'
      - '**/temp/**'

# Opções globais (nível raiz)
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

### Opções

#### Opções da regra (`rules.dead_code`)

- `exclude`: Uma lista de padrões glob para ignorar ao detectar código morto. Arquivos que correspondam a esses padrões serão tratados como se não existissem para fins de análise de dependência de entrada.

#### Opções globais (nível raiz)

- `entry_points`: Pontos de entrada globais que nunca devem ser relatados como código morto.

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-code': 'warn',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.
