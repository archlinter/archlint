# Estado Mutável Compartilhado

**ID:** `shared_state` | **Gravidade:** Medium (default)

Identifica variáveis exportadas que são mutáveis (ex: `export let ...` ou `export var ...`).

## Por que isso é um smell

O estado mutável global ou compartilhado é uma fonte comum de bugs extremamente difíceis de rastrear. Torna o comportamento de um módulo imprevisível e dependente da ordem de execução.

## Como corrigir

- **Use Const**: Exporte apenas constantes.
- **Encapsule**: Use uma classe ou uma função para gerenciar o estado e fornecer acesso controlado via métodos.
- **Use um Gerenciador de Estado**: Se o estado realmente precisar ser compartilhado, use uma biblioteca de gerenciamento de estado adequada (Redux, Zustand, etc.).
