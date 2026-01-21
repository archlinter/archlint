---
layout: home
title: Archlint — Linter de Arquitetura para TypeScript & JavaScript
description: "Detector rápido de problemas arquiteturais baseado em AST para projetos TypeScript/JavaScript. Pare a degradação da arquitetura com mais de 28 detectores e análise extremamente rápida."

hero:
  name: 'archlint'
  text: 'Nós não corrigimos sua arquitetura. Nós impedimos que ela piore.'
  tagline: Detector rápido de problemas arquiteturais baseado em AST para projetos TypeScript/JavaScript.
  image:
    src: /logo.svg
    alt: archlint logo
  actions:
    - theme: brand
      text: Primeiros Passos
      link: /pt/getting-started/
    - theme: alt
      text: Ver no GitHub
      link: https://github.com/archlinter/archlint

features:
  - title: 28+ Detectores
    details: De dependências cíclicas a Módulos Deus e violações de camadas. Construído com Rust e oxc para máximo desempenho.
  - title: Modo Diff
    details: Filosofia de melhoria progressiva (Enfoque Ratchet). Bloqueie o estado atual e falhe apenas em novas regressões arquiteturais.
  - title: Ciente do Framework
    details: Presets integrados para NestJS, Next.js, React e oclif. Conhece os padrões arquiteturais do seu framework.
  - title: Extremamente Rápido
    details: Analisa mais de 200 arquivos em menos de 5 segundos. Processamento paralelo e cache inteligente baseado em conteúdo.
  - title: Insights Acionáveis
    details: Cada relatório inclui pontuações de severidade, explicações claras e recomendações de refatoração.
  - title: Pronto para Integração
    details: Plugin ESLint, GitHub Actions, GitLab CI e até um servidor MCP para seu assistente de codificação AI.
---

## Por que archlint?

Codebases modernas tornam-se complexas rapidamente. archlint ajuda você a detectar problemas arquiteturais cedo, antes que se tornem dívida técnica.

```bash
# Capture regressões no seu PR
npx -y @archlinter/cli diff HEAD~1 --explain
```

```
🔴 REGRESSION: New cycle detected

  src/orders/service.ts → src/payments/processor.ts → src/orders/service.ts

  Why this is bad:
    Circular dependencies create tight coupling between modules.
    Changes in one module can cause unexpected failures in the other.

  How to fix:
    Extract shared logic into a separate module, or use dependency injection.
```
