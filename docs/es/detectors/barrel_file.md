# Abuso de Archivos Barrel

**ID:** `barrel_file` | **Severidad:** Media (por defecto)

Los archivos barrel (como un `index.ts` que simplemente reexporta todo) están pensados para simplificar los imports, pero a menudo se convierten en un agujero negro arquitectónico.

## Por qué esto es un problema

- **Fábrica de dependencias circulares**: Los barrels gigantes son la causa #1 de esas molestas dependencias circulares indirectas que son imposibles de rastrear.
- **Importar el mundo entero**: Cuando importas una pequeña constante de un barrel masivo, el bundler suele terminar arrastrando cada módulo que ese barrel referencia.
- **Te frena**: Hacen que la indexación del IDE se arrastre y pueden inflar tu bundle de producción si el tree-shaking no es perfecto.

## Configuración

```yaml
rules:
  barrel_file:
    severity: high
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
