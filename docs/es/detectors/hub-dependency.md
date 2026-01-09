# Dependencia Hub

**ID:** `hub_dependency` | **Severidad:** Media (predeterminado)

Identifica paquetes externos que son importados por demasiados archivos en tu proyecto, creando un punto central de fallo.

## Por qué esto es un problema

Cuando tu proyecto depende demasiado de una sola biblioteca externa, se vuelve difícil reemplazar o actualizar esa biblioteca. También sugiere que podrías estar filtrando detalles de infraestructura en la lógica de tu aplicación.

## Configuración

```yaml
rules:
  hub_dependency:
    severity: warn
    min_dependants: 20
    ignore_packages:
      - 'react'
      - 'lodash'
      - 'typescript'
```

### Opciones

- `min_dependants` (predeterminado: 20): El número mínimo de archivos que importan un paquete para activar este smell.
- `ignore_packages`: Una lista de nombres de paquetes o patrones glob para ignorar.

## Cómo corregir

Identifica por qué el paquete se usa tan ampliamente. Si es una biblioteca de utilidades como `lodash`, considera si realmente necesitas todas esas importaciones o si puedes usar características nativas del lenguaje. Para bibliotecas de infraestructura, usa el **Patrón Adapter** para aislar la dependencia.
