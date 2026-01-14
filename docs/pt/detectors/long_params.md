# Lista de Parâmetros Longa

**ID:** `long_params` | **Gravidade:** Low (default)

Identifica funções ou métodos que possuem parâmetros demais.

## Por que isso é um smell

Funções com muitos parâmetros são difíceis de usar e de ler. Elas geralmente indicam que a função está fazendo demais ou que alguns parâmetros deveriam ser agrupados em um objeto.

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
