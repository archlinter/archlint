# Abuso de Archivos Barrel

**ID:** `barrel_file` | **Severidad:** Media (por defecto)

Los archivos barrel (por ejemplo, archivos `index.ts` que solo reexportan otros archivos) pueden volverse problemáticos cuando crecen demasiado o incluyen demasiadas exportaciones no relacionadas.

## Por qué esto es un problema

- **Dependencias Circulares**: Los archivos barrel grandes son una causa común de dependencias circulares indirectas.
- **Acoplamiento Innecesario**: Importar una sola cosa de un archivo barrel grande puede hacer que el bundler incluya muchos módulos no relacionados.
- **Rendimiento**: Puede ralentizar tanto el desarrollo (indexación del IDE) como la producción (tamaño del bundle/tiempo de carga).

## Configuración

```yaml
rules:
  barrel_file:
    severity: error
    max_reexports: 10
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-barrel-abuse': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

## Cómo solucionarlo

- Evita los archivos barrel "atrapa-todo" en la raíz de directorios grandes.
- Prefiere importaciones directas si un archivo barrel está causando problemas.
- Agrupa las exportaciones en archivos barrel más pequeños y específicos.
