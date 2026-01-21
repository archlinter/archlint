---
title: Acoplamiento de Proveedor
description: "Detecta módulos demasiado acoplados a bibliotecas externas específicas, haciendo las migraciones futuras difíciles y las pruebas más complicadas."
---

# Acoplamiento de Proveedor (Vendor Coupling)

**ID:** `vendor_coupling` | **Severidad:** Medium (default)

Identifica módulos que están demasiado estrechamente acoplados a una biblioteca o framework externo específico.

## Por qué esto es un problema

Si decides cambiar la biblioteca en el futuro, tendrás que cambiar el código en muchos lugares. También hace que las pruebas sean más difíciles porque tienes que simular (mock) la biblioteca externa en todas partes.

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
