---
title: Complexidade Cognitiva
description: "Detecta funções que são difíceis de entender devido ao aninhamento profundo e lógica complexa, ajudando a reduzir a carga mental e o risco de manutenção."
---

# Complexidade Cognitiva

**ID:** `cognitive_complexity` | **Gravidade:** Média (padrão)

Este detector identifica funções com alta Complexidade Cognitiva. A Complexidade Cognitiva mede o quão difícil é entender o código, em vez de apenas quantos caminhos ele possui.

## Por que isso é um problema

- **Alta Carga Mental**: Lógica profundamente aninhada e expressões booleanas complexas tornam difícil para os desenvolvedores manterem o estado em suas cabeças.
- **Risco de Manutenção**: Código difícil de entender é propenso a erros durante a modificação.
- **Bugs Ocultos**: Erros de lógica frequentemente se escondem em estruturas profundamente aninhadas.

## Como é calculado

A Complexidade Cognitiva é calculada com base em:

1.  **Incrementos Estruturais**: `if`, `else`, `switch`, `for`, `while`, `do-while`, `catch`, operadores ternários e sequências lógicas.
2.  **Penalidade de Aninhamento**: Os incrementos para estruturas de controle aumentam com base em seu nível de aninhamento.
3.  **Casos Especiais**: `switch` conta apenas uma vez para todo o bloco, independentemente do número de casos.

## Como corrigir

1.  **Achatar a Lógica**: Use guard clauses (retornos antecipados) para reduzir o aninhamento.
2.  **Extrair Método**: Mova blocos aninhados ou condições complexas para funções pequenas e focadas.
3.  **Simplificar Expressões**: Divida condições booleanas complexas em variáveis ou funções intermediárias.
4.  **Substituir Ifs Aninhados**: Considere usar uma tabela de busca ou o padrão Strategy.

## Configuração

```yaml
rules:
  cognitive_complexity:
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
      '@archlinter/no-high-cognitive-complexity': 'warn',
    },
  },
];
```
