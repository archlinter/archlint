---
title: Presets Personalizados
description: "Aprende cómo crear presets personalizados basados en YAML para entender patrones específicos de frameworks y reducir falsos positivos en el análisis de archlint."
---

# Presets de Frameworks

archlint usa presets basados en YAML para entender patrones específicos de frameworks y reducir los falsos positivos.

## Cómo funciona

archlint detecta automáticamente frameworks analizando las dependencias en `package.json` y los archivos de configuración. También puedes extender explícitamente presets en tu `.archlint.yaml`:

```yaml
extends:
  - nestjs
  - ./my-company-preset.yaml
```

## Presets Integrados

- **nestjs**: Para aplicaciones NestJS.
- **nextjs**: Para proyectos Next.js.
- **react**: Para bibliotecas y aplicaciones React.
- **oclif**: Para herramientas CLI construidas con oclif.

## Presets Personalizados

Un archivo de preset es un archivo de configuración estándar de archlint con una sección adicional `detect` para el autodiscovery.

### Estructura

```yaml
name: my-framework
version: 1

# Reglas para autodetección
detect:
  packages:
    any_of: ['my-core-pkg']
  files:
    any_of: ['my-framework.config.js']

# Reglas globales
rules:
  layer_violation: high
  dead_symbols:
    ignore_methods: ['onInit', 'onDestroy']
  vendor_coupling:
    ignore_packages: ['my-framework/*']

# Sobrescrituras específicas por ruta
overrides:
  - files: ['**/*.controller.ts']
    rules:
      lcom: off

# Patrones para análisis de código muerto
entry_points:
  - '**/*.controller.ts'
```

### Cargar Presets Personalizados

Puedes cargar presets desde archivos locales o URLs:

```yaml
extends:
  - ./presets/shared.yaml
  - https://raw.githubusercontent.com/org/archlint-presets/main/standard.yaml
```

## Lógica de Fusión

Los presets se fusionan en el orden en que se especifican. La prioridad es:

1. Configuración del usuario en `.archlint.yaml` (la más alta)
2. Presets en la lista `extends`
3. Presets detectados automáticamente
4. Configuración predeterminada de archlint (la más baja)

Para configuraciones basadas en listas (como `entry_points` o `ignore_packages` dentro de las reglas), archlint realiza una unión de todos los valores. Las reglas y las sobrescrituras se fusionan recursivamente.
