# Alto Acoplamiento

**ID:** `high_coupling` | **Severity:** Medium (default)

El alto acoplamiento ocurre cuando un módulo depende de demasiados otros módulos (alto Fan-out).

## Por qué esto es un "smell"

- **Rigidez**: Un cambio en cualquiera de las dependencias podría requerir un cambio en este módulo.
- **Fragilidad**: Es más probable que el módulo se rompa cuando cambia cualquiera de sus dependencias.
- **Difícil de Probar**: Requiere muchos mocks para aislarlo en las pruebas unitarias.

## Cómo solucionar

1. **Extraer Responsabilidades**: Si un módulo tiene demasiadas dependencias, es probable que esté haciendo demasiado.
2. **Usar Abstracciones**: Dependa de una interfaz o una fachada en lugar de muchas implementaciones concretas.

## Configuración

```yaml
thresholds:
  high_coupling:
    max_dependencies: 15
```
