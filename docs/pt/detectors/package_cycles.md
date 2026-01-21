---
title: Ciclos de Pacotes
description: "Detecta dependências circulares entre pacotes inteiros que impedem versionamento adequado e indicam falhas sérias de modularidade."
---

# Ciclos de Pacotes

**ID:** `package_cycles` | **Gravidade:** High (default)

Detecta dependências circulares entre pacotes inteiros (pastas com `package.json` ou limites de módulos lógicos).

## Por que isso é um smell

Dependências circulares no nível do pacote são ainda mais graves do que ciclos no nível do arquivo. Elas impedem o versionamento adequado, tornam impossível publicar pacotes de forma independente e indicam uma falha séria na modularidade do sistema.

## Como consertar

Reavalie os limites entre seus pacotes. Muitas vezes, um ciclo de pacote significa que dois pacotes deveriam, na verdade, ser um só, ou que um terceiro pacote deveria ser extraído para conter o código compartilhado.
