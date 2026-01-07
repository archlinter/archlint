---
title: Primeros Pasos
description: Conoce la filosofía y las características clave de archlint, un detector de code smells de arquitectura basado en AST para TypeScript y JavaScript.
---

# Introducción

archlint es un detector de code smells de arquitectura basado en AST para proyectos TypeScript y JavaScript. Está diseñado para ayudar a los equipos a mantener una base de código saludable al prevenir regresiones arquitectónicas.

## Filosofía

### Enfoque Ratchet (mejora progresiva)

El mayor desafío con la deuda arquitectónica es su volumen. Si una herramienta informa de 500 dependencias cíclicas el primer día, es probable que el equipo lo ignore. archlint se centra en el **diff**. Bloquea el estado actual y solo falla en tu CI si introduces un _nuevo_ problema arquitectónico o empeoras uno existente.

### Explicar, no solo informar

Saber que tienes un "Módulo Dios" es solo la mitad de la batalla. archlint proporciona contexto: por qué se considera un defecto arquitectónico, cómo afecta a tu base de código y sugerencias para la refactorización.

### Sin complicaciones

Sin servidores que configurar ni bases de datos que mantener. Es una herramienta de CLI que se ejecuta en segundos, respeta tu `.gitignore` y se integra en cualquier pipeline de CI/CD con un solo comando.

## Características clave

- **28+ Detectores**: Cubriendo dependencias, diseño de módulos, complejidad y patrones de diseño.
- **Rápido**: Construido con Rust y el analizador `oxc`.
- **Consciente del Framework**: Inteligencia integrada para NestJS, Next.js, React y más.
- **Visual**: Genera informes con diagramas Mermaid para dependencias cíclicas.
- **Integración**: Plugin de ESLint para retroalimentación en tiempo real y un servidor MCP para refactorización asistida por IA.
