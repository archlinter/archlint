# Violación de Capa

**ID:** `layer_violation` | **Severidad:** High (default)

La violación de capa ocurre cuando tu "arquitectura limpia" empieza a tener goteras. Es cuando tu lógica de negocio de alto nivel (Domain) empieza a preguntar sobre tablas de la base de datos o endpoints de API (Infrastructure).

## Por qué esto es un problema

- **Abstracciones con goteras**: A tu lógica de negocio no debería importarle si usas Postgres o un archivo JSON. Cuando las capas gotean, pierdes esa libertad.
- **Tests frágiles**: No deberías necesitar levantar un mock de base de datos solo para probar una simple regla de negocio.
- **Fricción al cambiar**: ¿Quieres cambiar tu librería de logging? Lástima, la has importado directamente en el núcleo de tu dominio y ahora tienes que refactorizarlo todo.

## Configuración

Debe definir sus capas en `.archlint.yaml`:

```yaml
rules:
  layer_violation:
    layers:
  - name: domain
    path: ['**/domain/**']
    allowed_imports: [] # Domain no importa nada

  - name: application
    path: ['**/application/**']
    allowed_imports: ['domain']

  - name: infrastructure
    path: ['**/infrastructure/**']
    allowed_imports: ['domain', 'application']
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

## Cómo solucionar

1. **Inversión de Dependencia**: Defina una interfaz en la capa superior (Domain) e impleméntela en la capa inferior (Infrastructure).
2. **Refactorizar**: Mueva el código mal colocado a la capa adecuada.
