# Soporte para Frameworks

archlint no es solo un linter genérico; comprende los patrones arquitectónicos de los frameworks más populares y ajusta su análisis en consecuencia.

## Cómo funciona

archlint detecta automáticamente qué frameworks se utilizan en tu proyecto analizando el archivo `package.json` y la estructura de archivos. También puedes cargar explícitamente presets en tu `.archlint.yaml`:

```yaml
extends:
  - nestjs
  - react
```

## Beneficios de la Sensibilidad al Framework

- **Reducción de Falsos Positivos**: Algunos patrones que son "smells" en general (como el alto acoplamiento) son necesarios y esperados en ciertos contextos de frameworks (como los módulos de NestJS).
- **Puntos de Entrada Inteligentes**: Identifica automáticamente controladores, páginas y hooks como puntos de entrada para el análisis de código muerto (dead code).
- **Detectores Relevantes**: Deshabilita los detectores que no tienen sentido para un framework específico (como LCOM para los componentes de React).

## Frameworks Soportados

- [NestJS](/es/frameworks/nestjs)
- [Next.js](/es/frameworks/nextjs)
- [React](/es/frameworks/react)
- [oclif](/es/frameworks/oclif)

## Uso Avanzado

- [Presets Personalizados](/es/frameworks/custom-presets)
