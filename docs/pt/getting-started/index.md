---
title: Primeiros Passos
description: "Conheça a filosofia e os principais recursos do archlint, um detector de code smells de arquitetura baseado em AST para TypeScript e JavaScript."
---

# Introdução

archlint é um detector de code smells de arquitetura baseado em AST para projetos TypeScript e JavaScript. Ele foi projetado para ajudar as equipes a manter uma base de código saudável, evitando regressões arquiteturais.

## Filosofia

### Enfoque Ratchet (melhoria progressiva)

O maior desafio com a dívida arquitetural é o seu volume. Se uma ferramenta relata 500 dependências cíclicas no primeiro dia, a equipe provavelmente irá ignorá-la. O archlint foca no **diff**. Ele bloqueia o estado atual e só falha no seu CI se você introduzir um _novo_ problema arquitetural ou piorar um já existente.

### Explicar, não apenas relatar

Saber que você tem um "Módulo Deus" é apenas metade da batalha. O archlint fornece contexto: por que é considerado um defeito arquitetural, como ele impacta sua base de código e sugestões para refatoração.

### Sem complicações

Sem servidores para configurar ou bancos de datos para manter. É uma ferramenta CLI que roda em segundos, respeita o seu `.gitignore` e pode ser integrada em qualquer pipeline de CI/CD com um único comando.

## Principais Recursos

- **28+ Detectores**: Cobrindo dependências, design de módulo, complexidade e padrões de design.
- **Rápido**: Construído com Rust e o parser `oxc`.
- **Ciente do Framework**: Inteligência integrada para NestJS, Next.js, React e muito mais.
- **Visual**: Gera relatórios com diagramas Mermaid para dependências cíclicas.
- **Integração**: Plugin ESLint para feedback em tempo real e um servidor MCP para refatoração assistida por IA.
