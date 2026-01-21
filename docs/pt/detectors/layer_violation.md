---
title: Violação de Camada
description: "Detecta quando código em uma camada arquitetural importa incorretamente código de outra camada, quebrando abstrações e o Princípio da Responsabilidade Única."
---

# Violação de Camada

**ID:** `layer_violation` | **Gravidade:** High (default)

A violação de camada (Layer violation) ocorre quando o código em uma camada arquitetural importa código de uma camada que não deveria conhecer (por exemplo, a camada Domain importando da Infrastructure).

## Por que isso é um smell

- **Quebra a Abstração**: Detalhes de implementação interna vazam para a lógica de negócio de alto nível.
- **Dificuldade de Teste**: A lógica de negócio torna-se difícil de testar sem mocks para a infraestrutura (BD, API, etc.).
- **Rigidez**: Alterar um banco de dados ou biblioteca externa requer a alteração da lógica de negócio principal.

## Configuração

Você deve definir suas camadas em `.archlint.yaml`:

```yaml
rules:
  layer_violation:
    layers:
  - name: domain
    path: ['**/domain/**']
    allowed_imports: [] # Domain não importa nada

  - name: application
    path: ['**/application/**']
    allowed_imports: ['domain']

  - name: infrastructure
    path: ['**/infrastructure/**']
    allowed_imports: ['domain', 'application']
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.

## Como corrigir

1. **Inversão de Dependência**: Defina uma interface na camada superior (Domain) e implemente-a na camada inferior (Infrastructure).
2. **Refatorar**: Mova o código mal posicionado para a camada apropriada.
