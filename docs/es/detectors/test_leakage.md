---
title: Fuga de Pruebas
description: "Detecta código de producción que importa desde archivos de prueba, lo que puede llevar a un aumento del tamaño del bundle, riesgos de seguridad y builds rotos."
---

# Fuga de Pruebas (Test Leakage)

**ID:** `test_leakage` | **Severidad:** High (default)

Identifica código de producción que importa desde archivos de prueba o archivos de mock.

## Por qué esto es un problema

El código de producción nunca debería depender del código de prueba. Esto puede llevar a un aumento del tamaño del paquete, riesgos de seguridad y fallos en la construcción si los archivos de prueba se excluyen de la construcción de producción.

## Cómo corregir

- Mueve la lógica compartida del archivo de prueba a una ubicación segura para producción.
- Asegúrate de que tus rutas de importación sean correctas.
