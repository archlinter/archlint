# Soporte para React

Los componentes de React tienen características arquitectónicas diferentes a las clases o módulos tradicionales.

## Características Clave

- **Reconocimiento de Componentes**: Identifica componentes de React mediante patrones de nombres y uso de JSX.
- **LCOM Deshabilitado**: Deshabilita automáticamente el detector de Baja Cohesión (LCOM) para los componentes, ya que estos se centran inherentemente en el estado de la UI y el renderizado.
- **Análisis de Hooks**: Comprende que los hooks personalizados son puntos de entrada para la lógica de UI compartida.

## Configuración Recomendada

```yaml
framework: react
```
