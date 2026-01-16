---
title: Resumen de Detectores
description: Explora los más de 28 detectores de code smells de arquitectura en `archlint`, incluyendo dependencias cíclicas, violaciones de capas, módulos Dios y más.
---

# Resumen de detectores

`archlint` incluye más de 28 detectores integrados categorizados por el tipo de problema arquitectónico o de calidad de código que identifican.

::: tip
**Falsos positivos**: El análisis arquitectónico a veces puede producir falsos positivos, especialmente en proyectos con carga dinámica pesada, reflexión o contenedores complejos de Inyección de Dependencias (DI).
:::

## Problemas de dependencia

| Detector                                                 | ID                   | Descripción                                        | Por defecto |
| -------------------------------------------------------- | -------------------- | -------------------------------------------------- | ----------- |
| [Dependencias cíclicas](/es/detectors/cyclic_dependency) | `cyclic_dependency`  | Dependencias circulares entre archivos             | ✅          |
| [Clústeres de ciclos](/es/detectors/cycle_clusters)      | `cycle_clusters`     | Red compleja de dependencias circulares            | ✅          |
| [Ciclos de tipos](/es/detectors/circular_type_deps)      | `circular_type_deps` | Dependencias circulares solo de tipos              | ❌          |
| [Ciclos de paquetes](/es/detectors/package_cycles)       | `package_cycles`     | Dependencias cíclicas entre paquetes               | ❌          |
| [Violación de capas](/es/detectors/layer_violation)      | `layer_violation`    | Violaciones de las capas arquitectónicas definidas | ❌          |
| [Violación de SDP](/es/detectors/sdp_violation)          | `sdp_violation`      | Violaciones del Principio de Dependencias Estables | ❌          |

## Diseño de módulos y clases

| Detector                                               | ID                | Descripción                                        | Por defecto |
| ------------------------------------------------------ | ----------------- | -------------------------------------------------- | ----------- |
| [Módulo dios](/es/detectors/god_module)                | `god_module`      | Módulos con demasiadas responsabilidades           | ✅          |
| [Módulo hub](/es/detectors/hub_module)                 | `hub_module`      | Módulos "hub" altamente conectados                 | ❌          |
| [Baja cohesión](/es/detectors/lcom)                    | `lcom`            | Clases con baja cohesión interna (LCOM4)           | ❌          |
| [Alto acoplamiento](/es/detectors/high_coupling)       | `high_coupling`   | Módulos con demasiadas dependencias                | ❌          |
| [Módulo disperso](/es/detectors/module_cohesion)       | `module_cohesion` | Funcionalidad dispersa en demasiados archivos      | ❌          |
| [Envidia de funcionalidad](/es/detectors/feature_envy) | `feature_envy`    | Métodos que usan más otra clase que la suya propia | ❌          |

## Calidad del código y organización

| Detector                                                     | ID                    | Descripción                                             | Por defecto |
| ------------------------------------------------------------ | --------------------- | ------------------------------------------------------- | ----------- |
| [Código muerto](/es/detectors/dead_code)                     | `dead_code`           | Exportaciones no utilizadas                             | ✅          |
| [Símbolos muertos](/es/detectors/dead_symbols)               | `dead_symbols`        | Funciones y variables locales no utilizadas             | ✅          |
| [Tipos huérfanos](/es/detectors/orphan_types)                | `orphan_types`        | Tipos no conectados a la base de código                 | ✅          |
| [Abuso de barrel](/es/detectors/barrel_file)                 | `barrel_file`         | Archivos barrel grandes que causan acoplamiento         | ✅          |
| [Obsesión por primitivos](/es/detectors/primitive_obsession) | `primitive_obsession` | Uso excesivo de primitivos en lugar de tipos de dominio | ❌          |

## Complejidad y tamaño

| Detector                                                       | ID                      | Descripción                                | Por defecto |
| -------------------------------------------------------------- | ----------------------- | ------------------------------------------ | ----------- |
| [Complejidad ciclomática](/es/detectors/cyclomatic_complexity) | `cyclomatic_complexity` | Funciones con alta complejidad ciclomática | ✅          |
| [Complejidad cognitiva](/es/detectors/cognitive_complexity)    | `cognitive_complexity`  | Funciones con alta complejidad cognitiva   | ✅          |
| [Anidamiento profundo](/es/detectors/deep_nesting)             | `deep_nesting`          | Bloques de código profundamente anidados   | ✅          |
| [Muchos parámetros](/es/detectors/long_params)                 | `long_params`           | Funciones con demasiados parámetros        | ✅          |
| [Archivo grande](/es/detectors/large_file)                     | `large_file`            | Archivos fuente que son demasiado grandes  | ✅          |

## Patrones de cambio

| Detector                                               | ID                   | Descripción                                           | Por defecto |
| ------------------------------------------------------ | -------------------- | ----------------------------------------------------- | ----------- |
| [Cirugía de escopeta](/es/detectors/shotgun_surgery)   | `shotgun_surgery`    | Cambios que requieren modificación en muchos archivos | ❌          |
| [Interfaz Inestable](/es/detectors/unstable_interface) | `unstable_interface` | Interfaces públicas que cambian frecuentemente        | ❌          |

## Ejecución y seguridad

| Detector                                                              | ID                     | Descripción                                   | Por defecto |
| --------------------------------------------------------------------- | ---------------------- | --------------------------------------------- | ----------- |
| [Fuga de pruebas](/es/detectors/test_leakage)                         | `test_leakage`         | El código de prueba se filtra a producción    | ❌          |
| [Acoplamiento con proveedor](/es/detectors/vendor_coupling)           | `vendor_coupling`      | Acoplamiento estrecho con librerías externas  | ❌          |
| [Dependencia hub](/es/detectors/hub_dependency)                       | `hub_dependency`       | Dependencia excesiva de paquetes externos     | ❌          |
| [Importación con efecto secundario](/es/detectors/side_effect_import) | `side_effect_import`   | Importaciones que activan efectos secundarios | ✅          |
| [Estado mutable compartido](/es/detectors/shared_mutable_state)       | `shared_mutable_state` | Variables mutables exportadas                 | ❌          |

## Métricas arquitectónicas

| Detector                                                 | ID                 | Descripción                               | Por defecto |
| -------------------------------------------------------- | ------------------ | ----------------------------------------- | ----------- |
| [Violación de abstractez](/es/detectors/abstractness)    | `abstractness`     | Zonas de Dolor/Inutilidad (métrica I+A)   | ❌          |
| [Configuración dispersa](/es/detectors/scattered_config) | `scattered_config` | Configuración dispersa en muchos archivos | ❌          |
| [Clon de código](/es/detectors/code_clone)               | `code_clone`       | Código duplicado en el proyecto           | ✅          |
