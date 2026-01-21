---
title: Inveja de Recursos
description: "Detecta métodos que estão mais interessados nos dados de outra classe do que nos seus próprios, indicando uma violação de encapsulamento."
---

# Inveja de Recursos

**ID:** `feature_envy` | **Gravidade:** Medium (default)

A inveja de recursos (Feature envy) ocorre quando um método em uma classe parece mais interessado nos dados de outra classe do que nos dados de sua própria classe.

## Por que isso é um smell

Isso indica uma violação de encapsulamento. A lógica provavelmente está no lugar errado.

## Como corrigir

Mova o método (ou a parte do método que possui a inveja) para a classe cujos dados ele está usando.
