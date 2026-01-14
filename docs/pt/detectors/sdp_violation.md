# Princípio de Dependências Estáveis (SDP)

**ID:** `sdp_violation` | **Gravidade:** Medium (default)

O Princípio de Dependências Estáveis afirma que "as dependências entre pacotes devem estar na direção da estabilidade". Em outras palavras, módulos estáveis (difíceis de mudar) não devem depender de módulos instáveis (fáceis de mudar).

A estabilidade neste contexto é medida por quantos outros módulos dependem de um módulo (Fan-in) versus de quantos módulos ele depende (Fan-out).

## Por que isso é um smell

Quando um módulo estável—um do qual muitos outros componentes dependem—depende de um módulo instável, torna-se difícil de mudar. Qualquer modificação na dependência instável pode quebrar o módulo estável, que então repercute em todos os seus dependentes. Isso efetivamente "congela" o módulo instável ou torna todo o sistema frágil.

## Exemplos

### Ruim

Uma entidade de domínio principal (estável) dependendo de uma implementação específica de banco de dados ou um componente de UI que muda frequentemente (instável).

```typescript
// domain/user.ts (Estável, muitas coisas dependem de User)
import { UserPreferencesUI } from '../ui/user-prefs'; // Dependência instável

export class User {
  updateSettings(prefs: UserPreferencesUI) {
    // ...
  }
}
```

### Bom

O módulo estável depende de uma abstração (como uma interface) que muda raramente.

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

## Configuração

```yaml
rules:
  sdp_violation:
    severity: medium
    min_fan_total: 5
    instability_diff: 0.3
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

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

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.

## Como corrigir

Certifique-se de que seus módulos principais e estáveis não dependam de módulos voláteis. Use interfaces ou classes abstratas para desacoplá-los.
