---
title: Suporte ao React
description: "Análise especializada para componentes React, reconhecendo padrões de nomenclatura, desabilitando LCOM para componentes e entendendo custom hooks como pontos de entrada."
---

# Suporte ao React

Componentes React têm características arquiteturais diferentes das classes ou módulos tradicionais.

## Principais Recursos

- **Reconhecimento de Componentes**: Identifica componentes React por padrões de nomenclatura e uso de JSX.
- **LCOM Desabilitado**: Desabilita automaticamente o detector de Baixa Coesão (LCOM) para componentes, pois eles são inerentemente focados no estado da UI e renderização.
- **Análise de Hooks**: Entende que hooks personalizados são pontos de entrada para lógica de UI compartilhada.

## Configuração Recomendada

```yaml
extends:
  - react
```
