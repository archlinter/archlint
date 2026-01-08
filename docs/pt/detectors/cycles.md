# Dependências Cíclicas (Cyclic Dependencies)

**ID:** `cycles` | **Gravidade:** Crítica (padrão)

Dependências circulares ocorrem quando dois ou mais módulos dependem um do outro, direta ou indiretamente.

## Por que isso é um smell

- **Acoplamento Forte**: Os módulos são inseparáveis, tornando difícil reutilizá-los de forma independente.
- **Problemas de Inicialização**: Pode levar a imports "undefined" em tempo de execução se não for tratado com cuidado pelo bundler.
- **Dificuldade de Teste**: Difícil de mockar ou isolar um módulo sem trazer todo o ciclo junto.
- **Carga Cognitiva**: Mais difícil para os desenvolvedores entenderem o fluxo de dados e controle.

## Exemplos

### Ruim

```typescript
// orders.ts
import { processPayment } from './payments';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { createOrder } from './orders';
export const processPayment = () => {
  /* ... */
};
```

### Bom

Extraia a lógica compartilhada para um terceiro módulo.

```typescript
// types.ts
export interface Order {
  /* ... */
}

// orders.ts
import { Order } from './types';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { Order } from './types';
export const processPayment = (order: Order) => {
  /* ... */
};
```

## Configuração

```yaml
rules:
  cycles:
    severity: error
    exclude: ['**/*.test.ts']
```

## Como corrigir

1. **Extrair lógica compartilhada**: Mova as partes comuns para um novo módulo do qual ambos os módulos existentes dependam.
2. **Injeção de Dependência**: Passe as dependências como argumentos em vez de importá-las.
3. **Usar Eventos**: Use um barramento de eventos ou callbacks para desacoplar os módulos.

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-cycles': 'error',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.
