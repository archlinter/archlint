# Módulo Hub

**ID:** `hub_module` | **Severidad:** Medium (default)

Un "Módulo Hub" es como una intersección de tráfico concurrida sin semáforos. Es un módulo del que todos dependen, y que también depende de todos los demás, pero en realidad no _hace_ mucha lógica por sí mismo.

## Por qué esto es un problema

Los módulos hub son peligrosos puntos únicos de fallo. Como están en el centro de todo, son increíblemente frágiles. Un cambio minúsculo en un hub puede romper partes no relacionadas de tu app, convirtiéndolo en el archivo más aterrador de refactorizar en toda tu base de código. Es el "cuello de botella" definitivo para tu arquitectura.

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
    severity: medium
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
