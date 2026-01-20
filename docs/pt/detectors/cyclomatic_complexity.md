# Complexidade ciclomática (Cyclomatic Complexity)

**ID:** `cyclomatic_complexity` | **Gravidade:** Média (padrão)

A complexidade ciclomática mede quantos caminhos diferentes a execução do seu código pode seguir. Pense nela como o fator "espaguete" dos seus `if-else` e `switch`.

## Por que isso é um smell

- **Labirinto mental**: Cada `if`, `else` e `case` adiciona uma nova curva ao labirinto. Se uma função tem 20 caminhos, você pode apostar que um desenvolvedor acabará se perdendo mais cedo ou mais tarde.
- **Pesadelo para testes**: Para testar de verdade uma função complexa, você precisaria de um caso de teste para cada caminho possível. No mundo real, isso geralmente significa que alguns ramos nunca chegam a ser testados.
- **Efeito Borboleta**: Em funções muito complexas, mudar uma única linha de código pode ter consequências estranhas e imprevisíveis a cinco ramos de distância.

## Como corrigir

1. **Extrair Método (Extract Method)**: Divida a lógica complexa em funções menores e nomeadas.
2. **Cláusulas de Guarda (Guard Clauses)**: Use retornos antecipados para reduzir os níveis de aninhamento.
3. **Substituir Condicional por Polimorfismo**: Use objetos ou estratégias em vez de grandes blocos `switch` ou `if/else`.

## Configuração

```yaml
rules:
  cyclomatic_complexity:
    severity: medium
    max_complexity: 15
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-cyclomatic-complexity': 'warn',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.
