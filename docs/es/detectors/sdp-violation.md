# Principio de Dependencias Estables (SDP)

**ID:** `sdp_violation` | **Severity:** Medium (default)

El Principio de Dependencias Estables establece que "las dependencias entre paquetes deben estar en la dirección de la estabilidad". En otras palabras, los módulos estables (difíciles de cambiar) no deben depender de módulos inestables (fáciles de cambiar).

## Por qué es un smell

Si un módulo estable (uno del que muchos otros dependen) depende de un módulo inestable, el módulo estable se vuelve más difícil de cambiar porque cualquier cambio en el módulo inestable podría afectarlo, lo que a su vez afecta a todos sus dependientes.

## Cómo corregir

Asegúrate de que tus módulos principales y estables no dependan de módulos volátiles. Usa interfaces o clases abstractas para desacoplarlos.
