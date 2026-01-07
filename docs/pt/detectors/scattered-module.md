# Módulo Espalhado

**ID:** `scattered_module` | **Severity:** Medium (default)

Identifica um "módulo" (geralmente uma pasta ou um agrupamento lógico) onde os arquivos internos não estão bem conectados entre si, indicando que o módulo é apenas uma coleção aleatória de código.

## Por que isso é um "smell"

Um módulo deve ser coeso. Se suas partes internas não interagem entre si, provavelmente não é um módulo real e deve ser dividido ou reestruturado.

## Como consertar

Reavalie o propósito do módulo. Agrupe o código em módulos mais coesos ou mova as partes não relacionadas para onde elas são realmente usadas.
