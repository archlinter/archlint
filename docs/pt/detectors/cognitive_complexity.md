# Complexidade Cognitiva

**ID:** `cognitive_complexity` | **Gravidade:** Média (padrão)

A complexidade cognitiva não trata apenas de quantos caminhos o seu código tem; trata de quanto esforço o cérebro humano precisa fazer para realmente entendê-lo. É a diferença entre um código "tecnicamente correto" e um código "legível".

## Por que isso é um smell

- **Estouro de pilha mental**: Humanos não são bons em manter o fio da meada em cinco níveis de lógica aninhada e álgebra booleana complexa ao mesmo tempo. Quando a carga mental é muito alta, começamos a cometer erros.
- **Bugs invisíveis**: Bugs adoram se esconder nas sombras de `if`s aninhados e operadores ternários infinitos.
- **Atrito no review**: Se um dev sênior leva 20 minutos para entender uma função de 30 linhas durante um review de PR, ela está complexa demais.

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
