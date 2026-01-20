# Módulo Dios

**ID:** `god_module` | **Severidad:** High (default)

Un "Módulo Dios" es ese archivo en tu proyecto que todos temen tocar porque lo hace absolutamente todo. Suele empezar como un simple ayudante y termina convirtiéndose en un monstruo que maneja la autenticación, las consultas a la base de datos y el estado de la UI al mismo tiempo.

## Por qué esto es un problema

- **Pesadilla de responsabilidad única**: Cuando un módulo hace de todo, cualquier cambio —por pequeño que sea— se siente como jugar al Jenga con tu arquitectura.
- **Imán de conflictos**: Como es el "centro del universo", todos los desarrolladores del equipo están tocándolo constantemente, lo que garantiza conflictos infinitos al hacer merge.
- **Fragilidad**: Los cambios en una esquina del módulo pueden romper algo inesperadamente en la otra porque todo está conectado de forma implícita.
- **Dolor de cabeza al testear**: No deberías tener que simular una base de datos y un servicio de email solo para probar un simple formateador de texto.

## Criterios de Detección

`archlint` identifica los Módulos Dios basándose en:

- **Fan-in**: Número de otros módulos que dependen de él.
- **Fan-out**: Número de módulos de los que depende.
- **Churn**: Frecuencia de cambios en git.
- **Lines of Code**: Tamaño total del archivo.

## Cómo solucionar

1. **Identificar responsabilidades**: Enumere todas las diferentes tareas que realiza el módulo.
2. **Extraer módulos**: Divida el archivo en módulos más pequeños y enfocados.
3. **Patrón de Fachada**: Si el módulo actúa como coordinador, mantenga solo la lógica de coordinación y delegue el trabajo a los submódulos.

## Configuración

```yaml
rules:
  god_module:
    severity: high
    fan_in: 15
    fan_out: 15
    churn: 20
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-god-modules': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.
