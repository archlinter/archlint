---
title: Complexidade Ciclomática
description: "Detecta funções com muitos caminhos de ramificação que são difíceis de entender, testar e manter, aumentando o risco de bugs."
---

# Complexidade ciclomática (Cyclomatic Complexity)

**ID:** `cyclomatic_complexity` | **Gravidade:** Média (padrão)

Este detector identifica funções com alta Complexidade Ciclomática.

## Por que isso é um smell

- **Difícil de Entender**: Muitos caminhos de ramificação tornam o código difícil de seguir.
- **Propenso a Bugs**: Maior chance de esquecer casos de borda durante os testes.
- **Pesadelo de Manutenção**: Pequenas mudanças podem ter efeitos imprevisíveis devido à lógica complexa.

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
