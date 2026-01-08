# Soporte para Next.js

Los proyectos de Next.js tienen patrones de enrutamiento basados en archivos y de empaquetado únicos que archlint comprende.

## Características Clave

- **Sensible al Enrutamiento**: Reconoce automáticamente los archivos en los directorios `pages/` y `app/` como puntos de entrada.
- **Archivos Barrel**: Relaja las reglas de archivos barrel para patrones comunes de Next.js.
- **Componentes de Cliente/Servidor**: (Próximamente) Análisis especializado para la fuga de código exclusivo de servidor frente a exclusivo de cliente.

## Configuración Recomendada

```yaml
framework: nextjs

entry_points:
  - 'src/pages/**/*.tsx'
  - 'src/app/**/*.tsx'
```
