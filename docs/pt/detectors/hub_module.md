# Módulo Hub

**ID:** `hub_module` | **Gravidade:** Medium (default)

Um "Módulo Hub" é como um cruzamento movimentado sem semáforo. É um módulo do qual todo mundo depende, e que também depende de todo mundo, mas na verdade não _faz_ muita lógica por si só.

## Por que isso é um smell

Módulos hub são pontos únicos de falha perigosos. Como estão no centro de tudo, são incrivelmente frágeis. Uma pequena mudança em um hub pode quebrar partes não relacionadas do seu app, tornando-o o arquivo mais assustador de refatorar em toda a sua base de código. É o "gargalo" definitivo para sua arquitetura.

## Exemplos

### Ruim

Um módulo que apenas reexporta ou coordena muitos serviços não relacionados e é usado por toda a aplicação.

```typescript
// app-hub.ts
import { AuthService } from './auth';
import { ApiService } from './api';
import { LoggerService } from './logger';
import { ConfigService } from './config';
// ... mais 10 imports

export class AppHub {
  constructor(
    public auth: AuthService,
    public api: ApiService,
    public logger: LoggerService
    // ... mais 10 dependências
  ) {}
}
```

### Bom

Divida o hub em coordenadores específicos e focados ou use injeção de dependência no nível do consumidor para evitar um hub central.

```typescript
// auth-coordinator.ts (Focado em coordenação relacionada a auth)
import { AuthService } from './auth';
import { SessionStore } from './session';

export class AuthCoordinator {
  constructor(
    private auth: AuthService,
    private session: SessionStore
  ) {}
}
```

## Configuração

```yaml
rules:
  hub_module:
    severity: medium
    min_fan_in: 5
    min_fan_out: 5
    max_complexity: 5
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

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

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.

## Como corrigir

Quebre o hub! Identifique os diferentes caminhos de dados ou controle que passam pelo hub e extraia-os em módulos separados e mais focados.
