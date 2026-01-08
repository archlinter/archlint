# Módulo Hub

**ID:** `hub_module` | **Severidad:** Medium (default)

Un "Módulo Hub" (Hub Module) es un punto central en el grafo de dependencias, caracterizado tanto por un alto Fan-in (muchos dependientes) como un alto Fan-out (muchas dependencias), mientras contiene relativamente poca lógica interna.

## Por qué esto es un problema

Los módulos hub representan peligrosos "puntos únicos de falla" en tu arquitectura. Debido a que se encuentran en el centro de muchas rutas, se vuelven extremadamente frágiles. Un cambio menor en un módulo hub puede desencadenar un efecto dominó masivo en toda la base de código, haciéndolos difíciles y arriesgados de refactorizar.

## Ejemplos

### Mal

Un módulo que simplemente reexporta o coordina muchos servicios no relacionados y es utilizado por toda la aplicación.

```typescript
// app-hub.ts
import { AuthService } from './auth';
import { ApiService } from './api';
import { LoggerService } from './logger';
import { ConfigService } from './config';
// ... 10 más importaciones

export class AppHub {
  constructor(
    public auth: AuthService,
    public api: ApiService,
    public logger: LoggerService
    // ... 10 más dependencias
  ) {}
}
```

### Bien

Divide el hub en coordinadores específicos y enfocados, o usa inyección de dependencias a nivel del consumidor para evitar un hub central.

```typescript
// auth-coordinator.ts (Enfocado en coordinación relacionada con auth)
import { AuthService } from './auth';
import { SessionStore } from './session';

export class AuthCoordinator {
  constructor(
    private auth: AuthService,
    private session: SessionStore
  ) {}
}
```

## Configuración

```yaml
rules:
  hub_module:
    severity: warn
    min_fan_in: 5
    min_fan_out: 5
    max_complexity: 5
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-hub-modules': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

## Cómo solucionar

¡Rompa el hub! Identifique las diferentes rutas de datos o de control que pasan a través del hub y extráigalas en módulos separados y más enfocados.
