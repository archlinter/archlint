---
title: Suporte ao Next.js
description: "Entenda o roteamento baseado em arquivos do Next.js, reconheça diretórios pages/app como pontos de entrada e relaxe regras de barrel files para padrões comuns."
---

# Suporte ao Next.js

Projetos Next.js têm padrões únicos de roteamento baseado em arquivos e empacotamento que o archlint entende.

## Principais Recursos

- **Consciência de Roteamento**: Reconhece automaticamente arquivos nos diretórios `pages/` e `app/` como pontos de entrada.
- **Barrel Files**: Relaxa as regras de barrel files para padrões comuns do Next.js.
- **Componentes Client/Server**: (Em breve) Análise especializada para vazamento de código server-only vs client-only.

## Configuração Recomendada

```yaml
extends:
  - nextjs

entry_points:
  - 'src/pages/**/*.tsx'
  - 'src/app/**/*.tsx'
```
