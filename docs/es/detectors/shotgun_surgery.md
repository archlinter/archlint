# Cirugía de Escopeta (Shotgun Surgery)

**ID:** `shotgun_surgery` | **Severidad:** Medium (default)

La Cirugía de Escopeta es esa situación molesta donde un cambio "simple" requiere que toques 15 archivos diferentes. Es como intentar arreglar una fuga parcheando cien agujeros pequeños en lugar de reemplazar la tubería.

## Por qué esto es un problema

- **Alta fricción**: Cada pequeño cambio de requisitos se convierte en una operación mayor.
- **Fácil de olvidar un lugar**: Cuando la lógica está dispersa por todas partes, es casi seguro que olvides actualizar uno de esos archivos, llevando a "bugs fantasma".
- **Encapsulación rota**: Es señal de que una única responsabilidad ha escapado de su módulo y ahora se esconde en cada rincón de tu código.

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
