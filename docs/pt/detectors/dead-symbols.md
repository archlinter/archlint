# Símbolos Mortos (Dead Symbols)

**ID:** `dead_symbols` | **Gravidade:** Baixa (padrão)

Identifica funções, variáveis ou classes que são definidas dentro de um arquivo, mas nunca são usadas, nem mesmo localmente.

## Por que isso é um smell

É apenas poluição. Torna o arquivo mais difícil de ler e manter sem adicionar nenhum valor.

## Como corrigir

Delete os símbolos não utilizados.

## Configuração

```yaml
rules:
  dead_symbols:
    severity: low
    # Lista de nomes de métodos a ignorar (ex: métodos de ciclo de vida do framework)
    ignore_methods:
      - 'constructor'
    # Mapa de métodos de interface/classe a ignorar quando implementados
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate', 'defaultMessage']
```

::: tip
**Falsos Positivos**: A análise arquitetural pode, às vezes, produzir falsos positivos, especialmente em projetos com carregamento dinâmico pesado, reflexão ou contêineres complexos de Injeção de Dependência (DI).
:::

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-symbols': 'warn',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.
