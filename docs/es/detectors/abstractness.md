# Violación de Abstracción

**ID:** `abstractness_violation` | **Severidad:** Baja (por defecto)

Basado en las métricas de la "Secuencia Principal" de Robert C. Martin. Mide el equilibrio entre la estabilidad (I) y la abstracción (A). Un módulo debería ser o bien estable y abstracto, o inestable y concreto.

## Por qué esto es un problema (smell)

Los módulos que son estables y concretos están en la "Zona de Dolor" (difíciles de cambiar, pero otros dependen de ellos). Los módulos que son inestables y abstractos están en la "Zona de Inutilidad" (nadie depende de ellos, pero son abstractos).

## Cómo solucionarlo

Ajusta la abstracción del módulo (por ejemplo, introduciendo interfaces) o su estabilidad (cambiando quién depende de él).
