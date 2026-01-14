# Principio de Dependencias Estables (SDP)

**ID:** `sdp_violation` | **Severidad:** Medium (default)

El Principio de Dependencias Estables establece que "las dependencias entre paquetes deben estar en la dirección de la estabilidad". En otras palabras, los módulos estables (difíciles de cambiar) no deben depender de módulos inestables (fáciles de cambiar).

La estabilidad en este contexto se mide por cuántos otros módulos dependen de un módulo (Fan-in) versus de cuántos módulos depende (Fan-out).

## Por qué esto es un problema

Cuando un módulo estable—del que muchos otros componentes dependen—depende de un módulo inestable, se vuelve difícil de cambiar. Cualquier modificación en la dependencia inestable puede romper el módulo estable, lo que luego se propaga a todos sus dependientes. Esto efectivamente "congela" el módulo inestable o hace que todo el sistema sea frágil.

## Ejemplos

### Mal

Una entidad del dominio central (estable) dependiendo de una implementación de base de datos específica o un componente de UI que cambia frecuentemente (inestable).

```typescript
// domain/user.ts (Estable, muchas cosas dependen de User)
import { UserPreferencesUI } from '../ui/user-prefs'; // Dependencia inestable

export class User {
  updateSettings(prefs: UserPreferencesUI) {
    // ...
  }
}
```

### Bien

El módulo estable depende de una abstracción (como una interfaz) que cambia raramente.

```typescript
// domain/user.ts
export interface UserSettings {
  theme: string;
  notifications: boolean;
}

export class User {
  updateSettings(settings: UserSettings) {
    // ...
  }
}
```

## Configuración

```yaml
rules:
  sdp_violation:
    severity: medium
    min_fan_total: 5
    instability_diff: 0.3
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-sdp-violations': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

## Cómo corregir

Asegúrate de que tus módulos principales y estables no dependan de módulos volátiles. Usa interfaces o clases abstractas para desacoplarlos.
