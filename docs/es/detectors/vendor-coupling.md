# Acoplamiento de Proveedor (Vendor Coupling)

**ID:** `vendor_coupling` | **Severity:** Medium (default)

Identifica módulos que están demasiado estrechamente acoplados a una biblioteca o framework externo específico.

## Por qué es un smell

Si decides cambiar la biblioteca en el futuro, tendrás que cambiar el código en muchos lugares. También hace que las pruebas sean más difíciles porque tienes que simular (mock) la biblioteca externa en todas partes.

## Cómo corregir

Usa el **Patrón Adapter**. Crea una interfaz en tu dominio e impleméntala usando la biblioteca externa. El resto de tu código solo debería depender de tu interfaz.
