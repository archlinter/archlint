---
title: Ciclos de Tipos
description: "Detecta dependências circulares em importações apenas de tipos que indicam acoplamento arquitetural forte, mesmo que não causem problemas em tempo de execução."
---

# Ciclos de Tipos (Type Cycles)

**ID:** `circular_type_deps` | **Gravidade:** Média (padrão)

Semelhante a dependências circulares, mas especificamente para imports apenas de tipos (ex: `import type { ... }`).

## Por que isso é um smell

Embora ciclos apenas de tipos não causem problemas em tempo de execução no TypeScript, eles ainda indicam um acoplamento arquitetural forte. Eles tornam mais difícil separar módulos e ainda podem levar a grafos de dependência complexos e difíceis de entender.

## Como corrigir

Mova os tipos compartilhados para um módulo `types` comum ou um arquivo separado que não dependa dos módulos de implementação.
