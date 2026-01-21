---
title: Ciclos de Paquetes
description: "Detecta dependencias circulares entre paquetes completos que impiden el versionado adecuado e indican fallos serios de modularidad."
---

# Ciclos de Paquetes

**ID:** `package_cycles` | **Severidad:** High (default)

Detecta dependencias circulares entre paquetes completos (carpetas con `package.json` o límites de módulos lógicos).

## Por qué esto es un problema

Las dependencias circulares a nivel de paquete son aún más graves que los ciclos a nivel de archivo. Impiden un versionado adecuado, imposibilitan la publicación de paquetes de forma independiente e indican un fallo grave en la modularidad del sistema.

## Cómo solucionarlo

Reevalúa los límites entre tus paquetes. A menudo, un ciclo de paquete significa que dos paquetes deberían ser en realidad uno solo, o que se debería extraer un tercer paquete para contener el código compartido.
