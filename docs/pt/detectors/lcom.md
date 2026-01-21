---
title: Baixa Coesão (LCOM4)
description: "Mede o quão relacionados estão os métodos e campos de uma classe usando a métrica LCOM4 para detectar violações do Princípio da Responsabilidade Única."
---

# Baixa Coesão (LCOM4)

**ID:** `lcom` | **Gravidade:** Medium (default)

A coesão mede o quão intimamente relacionados estão os métodos e campos de uma classe. o `archlint` usa a métrica **LCOM4** (Lack of Cohesion of Methods).

## Por que isso é um smell

- **Violação do SRP**: A classe provavelmente está fazendo duas ou mais coisas não relacionadas.
- **Fragilidade**: Alterar uma parte da classe pode afetar partes não relacionadas.
- **Difícil de Reutilizar**: Você não pode usar uma parte da classe sem trazer lógica não relacionada.

## Como consertar

1. **Extract Class**: Divida a classe em duas ou mais classes menores, cada uma com uma única responsabilidade.
2. **Move Method**: Mova métodos que não usam o estado da classe para um local mais apropriado (por exemplo, um módulo de utilitários).

## Configuração

```yaml
rules:
  lcom:
    severity: medium
    max_lcom: 4
    min_methods: 3
```
