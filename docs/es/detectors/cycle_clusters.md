# Clústeres de Dependencias Cíclicas

**ID:** `cycle_clusters` | **Severity:** Critical (default)

Un clúster de dependencias cíclicas es un conjunto de dependencias circulares interconectadas que forman una red compleja de dependencias. A diferencia de los ciclos simples (A -> B -> A), los clústeres involucran múltiples ciclos que se superponen (por ejemplo, A -> B -> C -> A y B -> D -> C -> B).

## Por qué esto es un problema

- **Degradación Arquitectónica**: Los clústeres a menudo indican una falta de límites claros entre múltiples componentes.
- **Acoplamiento Extremo**: Todo el clúster debe ser tratado como una única unidad monolítica.
- **Imposibilidad de Aislamiento**: Es casi imposible cambiar o probar un módulo en el clúster sin afectar a todos los demás.
- **Pesadilla de Mantenimiento**: Los cambios en cualquier parte del clúster pueden tener efectos impredecibles en todos los módulos involucrados.

## Ejemplos

### Malo

Un grupo de módulos en un directorio "core" donde casi todos los módulos importan varios otros del mismo directorio, creando múltiples ciclos superpuestos.

### Bueno

Los módulos deben organizarse en una jerarquía o con un desacoplamiento claro basado en interfaces para garantizar que los ciclos no formen clústeres.

## Configuración

```yaml
rules:
  cycle_clusters:
    severity: high
    max_cluster_size: 5
```

## Cómo solucionar

1. **Romper el núcleo**: Identifique los módulos "núcleo" o "hub" que participan en múltiples ciclos y desacóplelos primero.
2. **Capas**: Imponga reglas estrictas de capas para evitar dependencias horizontales o hacia arriba.
3. **Refactorizar monolitos**: A menudo, los clústeres son una señal de que un solo módulo grande se dividió incorrectamente. Considere fusionar o volver a dividir siguiendo diferentes límites.
