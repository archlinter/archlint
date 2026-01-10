# Violación de Abstracción

**ID:** `abstractness` | **Severidad:** Baja (por defecto)

Este detector utiliza las métricas de la "Secuencia Principal" de Robert C. Martin para evaluar la relación entre la Estabilidad (I) y la Abstracción (A) de un módulo. El objetivo es asegurar que los módulos se sitúen cerca de la "Secuencia Principal"—una línea donde la abstracción aumenta a medida que aumenta la estabilidad.

## Por qué esto es un problema

- **Zona de Dolor**: Módulos que son altamente estables (muchas cosas dependen de ellos) pero muy concretos (sin abstracciones). Estos son extremadamente difíciles de cambiar debido a sus dependencias, pero su naturaleza concreta significa que _necesitarán_ cambiar.
- **Zona de Inutilidad**: Módulos que son altamente abstractos (muchas interfaces/clases abstractas) pero muy inestables (nadie depende de ellos). Estos proporcionan abstracciones que realmente no se están utilizando, añadiendo complejidad innecesaria.

## Cómo solucionarlo

- **En la Zona de Dolor**: Introduce abstracciones (interfaces, clases abstractas) para desacoplar la implementación del módulo de sus usuarios.
- **En la Zona de Inutilidad**: Considera hacer el módulo más concreto o eliminar las abstracciones no utilizadas para simplificar el código.

## Configuración

```yaml
rules:
  abstractness:
    severity: medium
```
