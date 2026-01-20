# Acoplamiento de Proveedor (Vendor Coupling)

**ID:** `vendor_coupling` | **Severidad:** Medium (default)

Identifica módulos que se han "casado" con una biblioteca o framework externo específico.

## Por qué esto es un problema

- **Vendor lock-in**: Si esa biblioteca queda deprecada o decides cambiar a una alternativa mejor, tendrás que reescribir la mitad de tu código.
- **Fricción en las pruebas**: No puedes testear tu lógica de negocio sin también arrastrar la pesada biblioteca externa y sus mocks.
- **Difícil de actualizar**: Estás atascado en la versión que la biblioteca soporte porque está tejida en cada archivo.

## Cómo corregir

Usa el **Patrón Adapter**. Crea una interfaz en tu dominio e impleméntala usando la biblioteca externa. El resto de tu código solo debería depender de tu interfaz.

## Configuración

```yaml
rules:
  vendor_coupling:
    severity: medium
    max_files_per_package: 10
    ignore_packages:
      - 'lodash'
      - 'rxjs'
      - '@nestjs/*'
```

### Opciones

- `max_files_per_package` (predeterminado: 10): El número máximo de archivos que pueden importar un paquete específico antes de que se reporte un smell.
- `ignore_packages`: Una lista de nombres de paquetes o patrones glob para ignorar.
