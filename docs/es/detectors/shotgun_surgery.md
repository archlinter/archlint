# Cirugía de Escopeta (Shotgun Surgery)

**ID:** `shotgun_surgery` | **Severidad:** Medium (default)

La Cirugía de Escopeta ocurre cuando un solo cambio en los requisitos te obliga a hacer muchos pequeños cambios en muchos módulos diferentes. `archlint` detecta esto analizando el historial de git para encontrar archivos que cambian frecuentemente juntos (alta frecuencia de co-cambio).

## Por qué esto es un problema

- **Alto Costo de Mantenimiento**: Cada funcionalidad o corrección de errores requiere tocar múltiples partes del sistema.
- **Propenso a Errores**: Es fácil olvidar uno de los muchos cambios requeridos, lo que lleva a bugs.
- **Encapsulación Deficiente**: Indica que una única responsabilidad está fragmentada a través de la base de código en lugar de estar encapsulada en un solo lugar.

## Cómo corregir

- **Consolidar Responsabilidades**: Usa **Move Method** o **Move Field** para reunir la lógica relacionada en un solo módulo.
- **Introduce Parameter Object**: Si múltiples módulos requieren el mismo conjunto de datos, agrúpalos en un solo objeto.
- **Replace Data Value with Object**: Si tienes muchos módulos manejando los mismos datos primitivos, encapsula esos datos y su comportamiento en una nueva clase.

## Configuración

```yaml
rules:
  shotgun_surgery:
    severity: medium
```
