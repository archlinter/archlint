---
title: Tipos Órfãos
description: "Encontra tipos ou interfaces que são definidos mas nunca usados, adicionando desordem e aumentando a carga cognitiva sem benefício."
---

# Tipos Órfãos

**ID:** `orphan_types` | **Gravidade:** Low (default)

Identifica tipos ou interfaces que são definidos, mas nunca usados como tipo para uma variável, parâmetro ou valor de retorno.

## Por que isso é um smell

Assim como o código morto, os tipos órfãos adicionam desordem e aumentam a carga cognitiva para os desenvolvedores sem fornecer nenhum benefício.

## Como consertar

Exclua os tipos não utilizados.
