---
layout: home
title: Archlint — Linter de Arquitectura para TypeScript & JavaScript
description: "Detector rápido de problemas arquitectónicos basado en AST para proyectos TypeScript/JavaScript. Detén la degradación de la arquitectura con más de 28 detectores и un análisis increíblemente rápido."

hero:
  name: 'archlint'
  text: 'No arreglamos tu arquitectura. Evitamos que empeore.'
  tagline: Detector rápido de problemas arquitectónicos basado en AST para proyectos TypeScript/JavaScript.
  image:
    src: /logo.svg
    alt: archlint logo
  actions:
    - theme: brand
      text: Primeros Pasos
      link: /es/getting-started/
    - theme: alt
      text: Ver en GitHub
      link: https://github.com/archlinter/archlint

features:
  - title: 28+ Detectores
    details: Desde dependencias cíclicas hasta módulos Dios y violaciones de capas. Construido con Rust y oxc para el máximo rendimiento.
  - title: Modo Diff
    details: Filosofía de mejora progresiva (enfoque Ratchet). Bloquea el estado actual y falla solo ante nuevas regresiones arquitectónicas.
  - title: Consciente del Framework
    details: Ajustes preestablecidos integrados para NestJS, Next.js, React y oclif. Conoce los patrones arquitectónicos de tu framework.
  - title: Increíblemente Rápido
    details: Analiza más de 200 archivos en menos de 5 segundos. Procesamiento en paralelo y caché inteligente basada en contenido.
  - title: Información Accionable
    details: Cada informe incluye puntuaciones de gravedad, explicaciones claras y recomendaciones de refactorización.
  - title: Listo para la Integración
    details: Plugin de ESLint, GitHub Actions, GitLab CI e incluso un servidor MCP para tu asistente de codificación de IA.
---

## ¿Por qué archlint?

Las bases de código modernas se vuelven complejas rápidamente. archlint te ayuda a detectar problemas arquitectónicos temprano, antes de que se conviertan en deuda técnica.

```bash
# Captura regresiones en tu PR
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
