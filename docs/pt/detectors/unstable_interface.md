---
title: Interface Instável
description: "Identifica módulos cuja API pública muda frequentemente enquanto muitos módulos dependem deles, causando efeitos cascata em toda a base de código."
---

# Interface Instável

**ID:** `unstable_interface` | **Gravidade:** Medium (default)

Identifica módulos cuja API pública (exports) muda frequentemente de acordo com o histórico do git, enquanto muitos outros módulos dependem dela.

## Por que isso é um smell

Uma interface instável causa um efeito dominó. Cada vez que a interface muda, todos os seus dependentes podem precisar ser atualizados, levando a muito trabalho desnecessário e possíveis bugs.

## Como corrigir

- **Estabilize a API**: Gaste mais tempo projetando a interface antes da implementação.
- **Use Versionamento**: Se possível, suporte múltiplas versões da interface simultaneamente durante uma transição.
- **Restrinja a Interface**: Exporte apenas o que for absolutamente necessário.
