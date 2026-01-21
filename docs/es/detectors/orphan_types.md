---
title: Tipos Huérfanos
description: "Encuentra tipos o interfaces que están definidos pero nunca se usan, añadiendo desorden y aumentando la carga cognitiva sin beneficio."
---

# Tipos Huérfanos

**ID:** `orphan_types` | **Severidad:** Low (default)

Identifica tipos o interfaces que están definidos pero nunca se usan como tipo para una variable, parámetro o valor de retorno.

## Por qué esto es un problema

Al igual que el código muerto, los tipos huérfanos añaden desorden y aumentan la carga cognitiva de los desarrolladores sin proporcionar ningún beneficio.

## Cómo solucionarlo

Elimina los tipos no utilizados.
