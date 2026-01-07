# Violación de Capa

**ID:** `layer_violation` | **Severity:** High (default)

La violación de capa (Layer violation) ocurre cuando el código en una capa arquitectónica importa código de una capa que no debería conocer (por ejemplo, la capa Domain importando de Infrastructure).

## Por qué esto es un "smell"

- **Rompe la Abstracción**: Los detalles de la implementación interna se filtran en la lógica de negocio de alto nivel.
- **Dificultad de Prueba**: La lógica de negocio se vuelve difícil de probar sin mocks para la infraestructura (BD, API, etc.).
- **Rigidez**: Cambiar una base de datos o una biblioteca externa requiere cambiar la lógica de negocio principal.

## Configuración

Debe definir sus capas en `.archlint.yaml`:

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Domain no importa nada

  - name: application
    paths: ['**/application/**']
    can_import: ['domain']

  - name: infrastructure
    paths: ['**/infrastructure/**']
    can_import: ['domain', 'application']
```

## Cómo solucionar

1. **Inversión de Dependencia**: Defina una interfaz en la capa superior (Domain) e impleméntela en la capa inferior (Infrastructure).
2. **Refactorizar**: Mueva el código mal colocado a la capa adecuada.
