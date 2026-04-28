---
layout: home
title: Archlint — Линтер архитектуры для TypeScript & JavaScript
description: 'Быстрый детектор архитектурных проблем на основе AST для TypeScript/JavaScript. 28+ детекторов и молниеносный анализ.'

hero:
  name: 'archlint'
  text: 'Мы не исправляем вашу архитектуру. Мы не даем ей стать хуже.'
  tagline: Быстрый детектор архитектурных проблем на основе AST для TypeScript/JavaScript проектов.
  image:
    src: /logo.svg
    alt: archlint logo
  actions:
    - theme: brand
      text: Начать работу
      link: /ru/getting-started/
    - theme: alt
      text: Посмотреть на GitHub
      link: https://github.com/archlinter/archlint

features:
  - title: 28+ Детекторов
    details: От циклических зависимостей до God-модулей и нарушений слоев. Написано на Rust и oxc для максимальной производительности.
  - title: Режим Diff
    details: Принцип непрерывного улучшения («храповик»). Фиксируйте текущее состояние и получайте отчеты только о новых архитектурных регрессиях.
  - title: Поддержка фреймворков
    details: Встроенные пресеты для NestJS, Next.js, Express, React, Vue, Angular и других. Знает архитектурные паттерны вашего фреймворка.
  - title: Молниеносно быстро
    details: Анализирует 200+ файлов менее чем за 5 секунд. Параллельная обработка и умное кэширование.
  - title: Понятные отчеты
    details: Каждый отчет включает оценки серьезности, четкие объяснения и рекомендации по рефакторингу.
  - title: Готов к интеграции
    details: Плагин ESLint, GitHub Actions, GitLab CI и MCP-сервер для вашего AI-помощника.
---

## Почему archlint?

Современные кодовые базы быстро становятся сложными. archlint помогает обнаруживать архитектурные проблемы на ранних стадиях, прежде чем они превратятся в технический долг.

```bash
# Поймайте регрессии в вашем PR
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
