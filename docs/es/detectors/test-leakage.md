# Fuga de Pruebas (Test Leakage)

**ID:** `test_leakage` | **Severity:** High (default)

Identifica código de producción que importa desde archivos de prueba o archivos de mock.

## Por qué es un smell

El código de producción nunca debería depender del código de prueba. Esto puede llevar a un aumento del tamaño del paquete, riesgos de seguridad y fallos en la construcción si los archivos de prueba se excluyen de la construcción de producción.

## Cómo corregir

- Mueve la lógica compartida del archivo de prueba a una ubicación segura para producción.
- Asegúrate de que tus rutas de importación sean correctas.
