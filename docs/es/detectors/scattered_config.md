# Configuración Dispersa

**ID:** `scattered_config` | **Severidad:** Low (default)

Identifica la configuración (como el acceso a variables de entorno) que está dispersa en muchos archivos diferentes en lugar de estar centralizada.

## Por qué esto es un problema

Centralizar la configuración facilita:

- Ver todas las opciones de configuración en un solo lugar.
- Proporcionar valores por defecto.
- Validar la configuración al inicio.
- Cambiar la fuente de la configuración (por ejemplo, de variables de entorno a un archivo o un gestor de secretos).

## Cómo solucionarlo

Crea un módulo `config` central que lea y valide todas las variables de entorno y las exporte como un objeto tipado.
