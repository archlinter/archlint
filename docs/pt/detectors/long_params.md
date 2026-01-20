# Lista de Parâmetros Longa

**ID:** `long_params` | **Gravidade:** Low (default)

Identifica funções que pedem informação demais de uma vez só.

## Por que isso é um smell

Funções com 10 parâmetros são confusas de chamar e ainda mais confusas de ler. O terceiro argumento era o `userId` ou o `orderId`? Quando você tem uma lista longa de argumentos, é sinal de que a função está fazendo demais ou que esses parâmetros pertencem juntos em um único objeto.

## Como consertar

- **Introduce Parameter Object**: Agrupe parâmetros relacionados em um único objeto ou interface.
- **Decompose Function**: Divida a função em funções menores que exijam menos parâmetros.

## Configuração

```yaml
rules:
  long_params:
    severity: low
    max_params: 5
    ignore_constructors: true
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-long-params': 'warn',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.
