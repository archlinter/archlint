---
title: Primer Escaneo
description: "Aprende cómo ejecutar tu primer escaneo arquitectónico con archlint, interpretar resultados y configurar la herramienta para tu proyecto."
---

# Primer Escaneo

Una vez instalado, realizar tu primer escaneo es sencillo.

## Ejecutar un escaneo básico

Navega a la raíz de tu proyecto y ejecuta:

```bash
npx @archlinter/cli scan
```

Por defecto, archlint hará lo siguiente:

1. Escaneará todos los archivos TypeScript y JavaScript en el directorio actual.
2. Respetará tu archivo `.gitignore`.
3. Utilizará los umbrales por defecto para los más de 28 detectores.
4. Mostrará un resumen en una tabla de colores de los code smells detectados.

## Guardar una instantánea (Snapshot)

Para utilizar el enfoque de "Trinquete", primero debes capturar el estado actual de tu arquitectura:

```bash
npx @archlinter/cli snapshot -o .archlint-baseline.json
```

Este archivo representa tu línea base (baseline) arquitectónica. Debes enviarlo a tu repositorio.

## Comprobar regresiones

Ahora, a medida que desarrollas, puedes comprobar si tus cambios han introducido algún problema arquitectónico nuevo:

```bash
npx @archlinter/cli diff .archlint-baseline.json
```

In un entorno de CI, normalmente compararías con la rama principal:

```bash
npx @archlinter/cli diff origin/main --fail-on medium
```

## ¿Qué sigue?

- [Más información sobre todos los detectores](/es/detectors/)
- [Configurar .archlint.yaml](/es/configuration/)
- [Integrar en CI/CD](/es/integrations/github-actions)
