---
title: Resumen de Detectores
description: Explora los más de 28 detectores de code smells de arquitectura en `archlint`, incluyendo dependencias cíclicas, violaciones de capas, módulos Dios y más.
---

# Resumen de Detectores

`archlint` incluye más de 28 detectores integrados categorizados por el tipo de problema arquitectónico o de calidad de código que identifican.

## Problemas de Dependencia

| Detector                                            | ID                   | Descripción                                        | Por defecto |
| --------------------------------------------------- | -------------------- | -------------------------------------------------- | ----------- |
| [Dependencias Cíclicas](/es/detectors/cycles)       | `cycles`             | Dependencias circulares entre archivos             | ✅          |
| [Ciclos de Tipos](/es/detectors/circular-type-deps) | `circular_type_deps` | Dependencias circulares solo de tipos              | ❌          |
| [Ciclos de Paquetes](/es/detectors/package-cycle)   | `package_cycles`     | Dependencias cíclicas entre paquetes               | ❌          |
| [Violación de Capas](/es/detectors/layer-violation) | `layer_violation`    | Violaciones de las capas arquitectónicas definidas | ❌          |
| [Violación de SDP](/es/detectors/sdp-violation)     | `sdp_violation`      | Violaciones del Principio de Dependencias Estables | ❌          |

## Diseño de Módulos y Clases

| Detector                                               | ID                | Descripción                                        | Por defecto |
| ------------------------------------------------------ | ----------------- | -------------------------------------------------- | ----------- |
| [Módulo Dios](/es/detectors/god-module)                | `god_module`      | Módulos con demasiadas responsabilidades           | ✅          |
| [Módulo Hub](/es/detectors/hub-module)                 | `hub_module`      | Módulos "hub" altamente conectados                 | ❌          |
| [Baja Cohesión](/es/detectors/lcom)                    | `lcom`            | Clases con baja cohesión interna (LCOM4)           | ❌          |
| [Alto Acoplamiento](/es/detectors/high-coupling)       | `high_coupling`   | Módulos con demasiadas dependencias                | ❌          |
| [Módulo Disperso](/es/detectors/scattered-module)      | `module_cohesion` | Funcionalidad dispersa en demasiados archivos      | ❌          |
| [Envidia de Funcionalidad](/es/detectors/feature-envy) | `feature_envy`    | Métodos que usan más otra clase que la suya propia | ❌          |

## Calidad del Código y Organización

| Detector                                                     | ID                    | Descripción                                             | Por defecto |
| ------------------------------------------------------------ | --------------------- | ------------------------------------------------------- | ----------- |
| [Código Muerto](/es/detectors/dead-code)                     | `dead_code`           | Exportaciones no utilizadas                             | ✅          |
| [Símbolos Muertos](/es/detectors/dead-symbols)               | `dead_symbols`        | Funciones y variables locales no utilizadas             | ✅          |
| [Tipos Huérfanos](/es/detectors/orphan-types)                | `orphan_types`        | Tipos no conectados a la base de código                 | ✅          |
| [Abuso de Barrel](/es/detectors/barrel-abuse)                | `barrel_file`         | Archivos barrel grandes que causan acoplamiento         | ✅          |
| [Obsesión por Primitivos](/es/detectors/primitive-obsession) | `primitive_obsession` | Uso excesivo de primitivos en lugar de tipos de dominio | ❌          |

## Complejidad y Tamaño

| Detector                                           | ID             | Descripción                                | Por defecto |
| -------------------------------------------------- | -------------- | ------------------------------------------ | ----------- |
| [Alta Complejidad](/es/detectors/complexity)       | `complexity`   | Funciones con alta complejidad ciclomática | ✅          |
| [Anidamiento Profundo](/es/detectors/deep-nesting) | `deep_nesting` | Bloques de código profundamente anidados   | ✅          |
| [Muchos Parámetros](/es/detectors/long-params)     | `long_params`  | Funciones con demasiados parámetros        | ✅          |
| [Archivo Grande](/es/detectors/large-file)         | `large_file`   | Archivos fuente que son demasiado grandes  | ✅          |

## Patrones de Cambio

| Detector                                               | ID                   | Descripción                                           | Por defecto |
| ------------------------------------------------------ | -------------------- | ----------------------------------------------------- | ----------- |
| [Cirugía de Escopeta](/es/detectors/shotgun-surgery)   | `shotgun_surgery`    | Cambios que requieren modificación en muchos archivos | ❌          |
| [Interfaz Inestable](/es/detectors/unstable-interface) | `unstable_interface` | Interfaces públicas que cambian frecuentemente        | ❌          |

## Ejecución y Seguridad

| Detector                                                              | ID                   | Descripción                                   | Por defecto |
| --------------------------------------------------------------------- | -------------------- | --------------------------------------------- | ----------- |
| [Fuga de Pruebas](/es/detectors/test-leakage)                         | `test_leakage`       | El código de prueba se filtra a producción    | ❌          |
| [Acoplamiento con Proveedor](/es/detectors/vendor-coupling)           | `vendor_coupling`    | Acoplamiento estrecho con librerías externas  | ❌          |
| [Dependencia Hub](/es/detectors/hub-dependency)                       | `hub_dependency`     | Dependencia excesiva de paquetes externos     | ❌          |
| [Importación con Efecto Secundario](/es/detectors/side-effect-import) | `side_effect_import` | Importaciones que activan efectos secundarios | ✅          |
| [Estado Mutable Compartido](/es/detectors/shared-mutable-state)       | `shared_state`       | Variables mutables exportadas                 | ❌          |

## Métricas Arquitectónicas

| Detector                                                 | ID                 | Descripción                               | Por defecto |
| -------------------------------------------------------- | ------------------ | ----------------------------------------- | ----------- |
| [Violación de Abstractez](/es/detectors/abstractness)    | `abstractness`     | Zonas de Dolor/Inutilidad (métrica I+A)   | ❌          |
| [Configuración Dispersa](/es/detectors/scattered-config) | `scattered_config` | Configuración dispersa en muchos archivos | ❌          |
| [Clon de Código](/es/detectors/code-clone)               | `code_clone`       | Código duplicado en el proyecto           | ✅          |
