# Clústeres de Dependencias Cíclicas

**ID:** `cycle_clusters` | **Severity:** Critical (default)

Un clúster de dependencias cíclicas es lo que ocurre cuando las dependencias circulares comienzan a reproducirse. No es solo un simple bucle "A depende de B, B depende de A"—es una red compleja donde una docena de módulos están todos enredados juntos.

## Por qué esto es un problema

- **Degradación Arquitectónica**: Es señal de que tus límites de módulos se han derrumbado completamente.
- **El efecto "Monolito"**: No puedes simplemente tomar un módulo del clúster; tienes que arrastrar todo el lío enredado. Es un paquete completo que no pediste.
- **Imposibilidad de Aislamiento**: ¿Quieres testear una sola función? Mala suerte, ahora estás mockeando la mitad de tu código porque todo está interconectado.
- **Pesadilla de Mantenimiento**: Cambiar un módulo en el clúster puede desencadenar un efecto mariposa impredecible que rompe algo al otro lado de la red.

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
